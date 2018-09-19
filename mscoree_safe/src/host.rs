#![allow(dead_code, non_snake_case)]
// host.rs - MIT License
//  MIT License
//  Copyright (c) 2018 Tyler Laing (ZerothLaw)
// 
//  Permission is hereby granted, free of charge, to any person obtaining a copy
//  of this software and associated documentation files (the "Software"), to deal
//  in the Software without restriction, including without limitation the rights
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//  copies of the Software, and to permit persons to whom the Software is
//  furnished to do so, subject to the following conditions:
// 
//  The above copyright notice and this permission notice shall be included in all
//  copies or substantial portions of the Software.
// 
//  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//  SOFTWARE.
use std::ptr;
use std::{thread, time};

use mscoree_sys::metahost::{CLSID_CLRMetaHost, CLRCreateInstance, ICLRMetaHost, ICLRRuntimeInfo, IID_ICLRMetaHost, IID_ICLRRuntimeInfo};
use mscoree_sys::mscoree::{ICLRRuntimeHost, ICLRControl, CLSID_CLRRuntimeHost, IID_ICLRRuntimeHost, IHostControl, IHostControlVtbl};
use mscoree_sys::c_wrapper::rusthostcontrol::{RustHostControl, RustHostControl_new};
use mscorlib_safe::BString;
use mscorlib_sys::system::_AppDomainManager;
use winapi::shared::minwindef::{BOOL,DWORD,  LPVOID, UINT};
use winapi::shared::ntdef::HANDLE;
use winapi::um::oaidl::ITypeInfo;

use wrappers::{PtrCtr, WrapperErrors, Sealed, RefCtr, RefCounted};

extern "system" {
    pub fn GetCurrentProcess() -> HANDLE;
}

pub struct MetaHost {
    inner: PtrCtr<ICLRMetaHost>, 
}

#[derive(Debug)]
pub enum RuntimeHostError {
    InitFailure,
    SetHostControl,
    GetCLRControl, 
    StartFailure
}

#[derive(Debug)]
pub enum HostControlError {
    GetAppDomainManager
}

#[derive(Debug)]
pub enum MetaHostError {
    InitFailure, 
    PtrCtr(WrapperErrors),
    RuntimeInfoInitFailure,
    RuntimeHost(RuntimeHostError),
}

pub enum RuntimeVersion {
    V4,
}

macro_rules! HANDLE_HRESULT {
    ($uns:expr, $err:expr) => {
        let hr = unsafe {$uns};
        match hr {
            0 => {}, 
            _ => {println!("HRESULT = 0x{:x}", hr); return Err($err);}
        }
    };
}

impl MetaHost {
    pub fn new() -> Result<MetaHost, MetaHostError> {
        let mut mh_ptr: *mut ICLRMetaHost = ptr::null_mut();
        HANDLE_HRESULT!{CLRCreateInstance(&CLSID_CLRMetaHost, &IID_ICLRMetaHost, &mut mh_ptr as *mut _ as *mut LPVOID), MetaHostError::InitFailure }

        let wrapped = PtrCtr::new_checked(mh_ptr);
        match wrapped {
            Ok(pc) => { Ok(MetaHost {inner: pc}) }, 
            Err(err) => { Err(MetaHostError::PtrCtr(err)) }
        }
    }

    pub fn runtime(&self, version: RuntimeVersion) -> Result<RuntimeInfo, MetaHostError> {
        match version {
            RuntimeVersion::V4 => {
                let bs = BString::from("v4.0.30319");
                let mut ri_ptr: *mut ICLRRuntimeInfo = ptr::null_mut();
                HANDLE_HRESULT!{(*self.inner.as_const()).GetRuntime(bs.as_sys(), &IID_ICLRRuntimeInfo, &mut ri_ptr as *mut _ as *mut LPVOID), MetaHostError::RuntimeInfoInitFailure}
                let wrapped = PtrCtr::new_checked(ri_ptr);
                match wrapped {
                    Ok(pc) => { Ok(RuntimeInfo::new_from(pc)) }, 
                    Err(err) => { Err(MetaHostError::PtrCtr(err)) }
                }
            }
        }
    }
}
impl Sealed for MetaHost {}
impl RefCounted for MetaHost {
    fn increment(&self) -> bool {
        unsafe {(*self.inner.as_const()).AddRef()};
        true
    }
    fn decrement(&self) -> bool {
        unsafe {(*self.inner.as_const()).Release()};
        true
    }
}

pub struct RuntimeInfo {
    inner: PtrCtr<ICLRRuntimeInfo>,
}

impl RuntimeInfo {
    pub(crate) fn new_from(obj: PtrCtr<ICLRRuntimeInfo>) -> RuntimeInfo {
       RuntimeInfo {inner: obj} 
    }

    pub fn runtime_host(&self) -> Result<RuntimeHost, MetaHostError> {
        let mut rh_ptr: *mut ICLRRuntimeHost = ptr::null_mut();
        HANDLE_HRESULT!{(*self.inner.as_const()).GetInterface(&CLSID_CLRRuntimeHost, &IID_ICLRRuntimeHost, &mut rh_ptr as *mut _ as *mut LPVOID ), MetaHostError::RuntimeHost(RuntimeHostError::InitFailure)}
        let wrapped = PtrCtr::new_checked(rh_ptr);
        match wrapped {
            Ok(pc) => Ok(RuntimeHost::new_from(pc)), 
            Err(err) => Err(MetaHostError::PtrCtr(err)),
        }
    }

    pub fn started(&self) -> bool {
        let mut vb: BOOL = 0;
        let mut dw: DWORD = 0;

        let hr = unsafe {
            (*self.inner.as_const()).IsStarted(&mut vb as *mut BOOL, &mut dw as *mut DWORD)
        };
        println!("HR: 0x{:x} | started: {} | dw: {}", hr, vb, dw);
        if hr != 0 {
            return false;
        }
        vb > 0
    }
}

pub struct RuntimeHost {
    inner: PtrCtr<ICLRRuntimeHost>,
}

impl RuntimeHost {
    pub fn new_from(obj: PtrCtr<ICLRRuntimeHost>) -> RuntimeHost {
        RuntimeHost{ inner: obj }
    }

    pub fn start(&mut self) -> Result<Box<HostControl>, MetaHostError> {
        let mut hc = HostControl::new();
        HANDLE_HRESULT!{(*self.inner.as_const()).SetHostControl(hc.inner.as_mut() as *mut HostControl as *mut IHostControl), MetaHostError::RuntimeHost(RuntimeHostError::SetHostControl) };
        HANDLE_HRESULT!{(*self.inner.as_const()).Start(), MetaHostError::RuntimeHost(RuntimeHostError::StartFailure)}
        Ok(Box::new(hc))
    }
}

pub struct DomainManager<'a> {
    inner: RefCtr<'a, _AppDomainManager> 
}

impl<'a> DomainManager<'a> {
    fn type_count(&self) -> u32 {
        println!("inside typecount");
        let mut t_count: UINT = 0;
        let hr = unsafe {
            (*self.inner.as_const()).GetTypeInfoCount(&mut t_count as *mut UINT)
        };
        println!("hresult {}", hr);
        t_count
    }

    fn type_info(&self) {

        let mut tinfo: *mut ITypeInfo = ptr::null_mut();
        let hr = unsafe {
            (*self.inner.as_const()).GetTypeInfo(0, 0, &mut tinfo as *mut *mut _ as *mut *mut ITypeInfo)
        };
        println!("hresult = {:x}", hr);
        println!("tinfo {:p}", tinfo);
    }
}

pub struct HostControl {
    inner: PtrCtr<RustHostControl>
}

impl HostControl {
    fn new() -> HostControl {
        let phc = unsafe{RustHostControl_new()};
        println!("RustHostControl {:p}", phc);
        HostControl{inner: PtrCtr::new_checked(phc).unwrap()}
    }
    pub fn domain_manager<'a>(&'a self) -> Result<DomainManager<'a>, HostControlError> {
        let mut padm: *mut _AppDomainManager = ptr::null_mut();
        let hr = unsafe {
            let hc = &(*self.inner.as_const());
            hc.GetAppDomainManager(&mut padm as *mut *mut _ as *mut *mut _AppDomainManager) 
        };
        println!("padm {:p}, hr: 0x{:x}", padm, hr);
        if hr != 0 {
            println!("{}", hr);
            return Err(HostControlError::GetAppDomainManager);
        }
        let p2 = ptr::NonNull::new(padm);
        match p2 {
            Some(p2) => Ok(DomainManager{inner: RefCtr::new(p2)}), 
            None => Err(HostControlError::GetAppDomainManager)
        }
    }
}

trace_macros!(false);

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn load_test() {
        let mut metahost = MetaHost::new().unwrap();
        let mut runtime = metahost.runtime(RuntimeVersion::V4).unwrap();
        let mut host = runtime.runtime_host().unwrap();
        let hc = host.start().unwrap();
        thread::sleep(time::Duration::from_secs(5));
        while !runtime.started(){
            println!("Sleeping...");
            thread::sleep(time::Duration::from_secs(10));
        }
        let dm = hc.domain_manager().unwrap();
        
        println!("dm obtained");
        dm.type_info();
    }
}