#![allow(dead_code, non_upper_case_globals, non_camel_case_types, non_snake_case)]
// vererror.rs - MIT License
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

use winapi::ctypes::{c_int, c_void};
use winapi::shared::minwindef::{DWORD, ULONG};

STRUCT!{struct _VerItem
{
	dwFlags: DWORD,	 // BYREF / BOXED etc.. see veritem.hpp
	pv: *mut c_void, // TypeHandle / MethodDesc * etc.
}}

STRUCT!{struct tag_VerError {
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
