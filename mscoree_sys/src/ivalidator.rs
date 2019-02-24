#![allow(dead_code, non_upper_case_globals, non_camel_case_types, non_snake_case)]
// ivalidator.h - MIT License
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

use winapi::shared::minwindef::BYTE;
use winapi::shared::ntdef::{LPWSTR, ULONG};
use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::SAFEARRAY;
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use crate::ivehandler::{IVEHandler, VEContext};

ENUM!{enum ValidatorFlags
{
    VALIDATOR_EXTRA_VERBOSE	= 0x1,
    VALIDATOR_SHOW_SOURCE_LINES	= 0x2,
    VALIDATOR_CHECK_ILONLY	= 0x4,
    VALIDATOR_CHECK_PEFORMAT_ONLY	= 0x8,
    VALIDATOR_NOCHECK_PEFORMAT	= 0x10,
    VALIDATOR_TRANSPARENT_ONLY	= 0x20,
}}

RIDL!{#[uuid(0x63DF8730, 0xDC81, 0x4062, 0x84, 0xA2, 0x1F, 0xF9, 0x43, 0xF5, 0x9F, 0xAC)]
interface IValidator(IValidatorVtbl): IUnknown(IUnknownVtbl){
    fn Validate(
        veh: *mut IVEHandler, 
        pAppDomain: *mut IUnknown, 
        ulFlags: ULONG, 
        ulMaxError: ULONG, 
        token: ULONG, 
        fileName: LPWSTR, 
        pe: *mut BYTE, 
        ulSize: ULONG,
    ) -> HRESULT, 
    fn FormatEventInfo(
        hVECode: HRESULT, 
        Context: VEContext, 
        msg: LPWSTR, 
        ulMaxLength: ULONG, 
        psa: *mut SAFEARRAY,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x63DF8730, 0xDC81, 0x4062, 0x84, 0xA2, 0x1F, 0xF9, 0x43, 0xF5, 0x9F, 0xDD)]
interface ICLRValidator(ICLRValidatorVtbl): IUnknown(IUnknownVtbl){
    fn Validate(
        veh: *mut IVEHandler,
        ulAppDomainId: ULONG, 
        ulFlags: ULONG, 
        ulMaxError: ULONG, 
        token: ULONG, 
        fileName: LPWSTR, 
        pe: *mut BYTE, 
        ulSize: ULONG,
    ) -> HRESULT,
    fn FormatEventInfo(
        hVECode: HRESULT, 
        Context: VEContext, 
        msg: LPWSTR, 
        ulMaxLength: ULONG, 
        psa: *mut SAFEARRAY,
    ) -> HRESULT,
}}