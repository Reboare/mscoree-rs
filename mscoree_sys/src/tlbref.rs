#![allow(dead_code, non_upper_case_globals, non_camel_case_types, non_snake_case)]
// tlbref.rs - MIT License
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

use winapi::shared::guiddef::GUID;
use winapi::shared::ntdef::{LCID, LPWSTR, USHORT};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::BSTR;
use winapi::shared::wtypesbase::LPCOLESTR;
use winapi::um::oaidl::{ITypeLib, SYSKIND};
use winapi::um::oleauto::REGKIND;
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

DEFINE_GUID!(IID_ITypeLibResolver, 0x8F026EDB, 0x785E, 0x4470, 0xA8, 0xE1, 0xB4, 0xE8, 0x4E, 0x9D, 0x17, 0x79);

RIDL!{#[uuid(0x8F026EDB, 0x785E, 0x4470, 0xA8, 0xE1, 0xB4, 0xE8, 0x4E, 0x9D, 0x17, 0x79)]
interface ITypeLibResolver(ITypeLibResolverVtbl): IUnknown(IUnknownVtbl){
    fn ResolveTypeLib(
        bstrSimpleName: BSTR, 
        tlbid: GUID, 
        lcid: LCID, 
        wMajorVersion: USHORT,
        wMinorVersion: USHORT,
        syskind: SYSKIND,
        pbstrResolvedTlbName: *mut BSTR,
    ) -> HRESULT,
}}

STDAPI!{fn LoadTypeLibWithResolver(
    szFile: LPCOLESTR, 
    regkind: REGKIND, 
    pTlbResolver: *mut ITypeLibResolver, 
    pptlib: *mut *mut ITypeLib,
) -> HRESULT}
STDAPI!{fn GetTypeLibInfo(
    szFile: LPWSTR, 
    pTypeLibID: *mut GUID, 
    pTypeLibLCID: *mut LCID, 
    pTypeLibPlatform: *mut SYSKIND, 
    pTypeLibMajorVer: *mut USHORT, 
    pTypeLibMinorVer: *mut USHORT,
) -> HRESULT}