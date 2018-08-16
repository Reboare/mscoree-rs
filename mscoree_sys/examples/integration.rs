use std::ptr;

extern crate mscoree_sys;

#[macro_use]
extern crate winapi;

use winapi::ctypes::c_void;

use mscoree_sys::metahost::{CLRCreateInstance, CLSID_CLRMetaHost, ICLRMetaHost};

DEFINE_GUID!{IID_ICLRMETAHOST, 
	0xD332DB9E, 0xB9B3, 0x4125, 0x82, 0x07, 0xA1, 0x48, 0x84, 0xF5, 0x32, 0x16}

fn main() {
    println!("A");
    let mut ppv: *mut ICLRMetaHost = ptr::null_mut();
    println!("B");
    let hr = unsafe {
        println!("C");
        CLRCreateInstance(&CLSID_CLRMetaHost, &IID_ICLRMETAHOST, &mut ppv as *mut *mut _ as *mut *mut c_void)
    };
    println!("D");
    println!("HRESULT = 0x{:x}, ptr = {:p}", hr, ppv);
    println!("E");
}