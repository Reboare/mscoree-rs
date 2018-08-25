#![feature(macro_literal_matcher)]
// lib.rs - MIT License
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

#[macro_use] extern crate winapi;
extern crate mscorlib_sys;
extern crate regex;
#[macro_use] extern crate lazy_static;
#[macro_use] pub mod macros;

pub mod activation;
pub mod clrdata;
pub mod cor;
pub mod cordebug;
pub mod corhdr;
pub mod corhlpr;
pub mod corprof;
pub mod corpub;
pub mod corsym;
pub mod iceefilegen;
pub mod isolation;
pub mod ivalidator;
pub mod ivehandler;
pub mod inspectable;
pub mod gchost;
pub mod metahost;
pub mod mscoree;
pub mod openum;
pub mod strongname;
pub mod tlbref;
pub mod vererror;

pub mod core {
    pub use winapi::Interface;
    pub use std::ops::Deref;
}