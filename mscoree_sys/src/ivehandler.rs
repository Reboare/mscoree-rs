#![allow(dead_code, non_upper_case_globals, non_camel_case_types, non_snake_case)]
// ivehandler.rs - MIT License
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

use winapi::ctypes::c_int;
use winapi::shared::ntdef::{ULONG};
use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::SAFEARRAY;
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

STRUCT!{struct tag_VerError
{
    flags: ULONG,
    opcode: ULONG, 
    uOffset: ULONG,
    Token: ULONG,
    item1_flags: ULONG,
    item1_data: *mut c_int,
    item2_flags: ULONG,
    item2_data: *mut c_int,
}}
pub type _VerError = tag_VerError;
pub type VEContext = _VerError;

RIDL!{#[uuid(0x856CA1B2, 0x7DAB, 0x11d3, 0xAC, 0xEC, 0x00, 0xC0, 0x4F, 0x86, 0xC3, 0x09)]
interface IVEHandler(IVEHandlerVtbl): IUnknown(IUnknownVtbl){
    fn VEHandler(
        VECode: HRESULT, 
        Context: VEContext, 
        psa: *mut SAFEARRAY,
    ) -> HRESULT, 
    fn SetReporterFtn(
        lfnPtr: i64,
    ) -> HRESULT, 
}}