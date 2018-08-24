// macros.rs - MIT License
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

use regex::Regex;

#[macro_export]
macro_rules! STDAPI {
    (fn $func:ident($($id:ident : $tpe:ty,)*) -> $return_type:ty) => {
        extern "system" { pub fn $func(
            $($id: $tpe,)*
        ) -> $return_type;}
    };
}
//pub type FLockClrVersionCallback = extern fn() -> HRESULT;
#[macro_export]
macro_rules! FUNC_PTR {
    ($name:ident($($p_name:ident : $p_ty:ty),*) -> $res:ty ) => {
        pub type $name = extern fn($($p_ty),*) -> $res;
    };
    ($name:ident($($p_name:ident : $p_ty:ty),*)) => {
        pub type $name = extern fn($($p_ty),*);
    };
}

lazy_static!{
    static ref UUID_RE: Regex = Regex::new(r"([\w\d]{8})-([\w\d]{4})-([\w\d]{4})-([\w\d]{2})([\w\d]{2})-([\w\d]{2})([\w\d]{2})([\w\d]{2})([\w\d]{2})([\w\d]{2})([\w\d]{2})").unwrap();
}

#[macro_export]
macro_rules! STR_TO_UUID {
    ($uuid:ident, $val:expr) => {
        lazy_static!{
            static ref $uuid = {
                let mut v = Vec::new();
                for cap in UUID_RE.captures_iter($val) {
                    for i in 1..cap.length() {
                        v.push(u32::from_str_radix(&cap[i], 16).unwrap());
                    }
                }
                v
            }; 
        }
    };
}