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

#[macro_export]
macro_rules! STDAPI {
    ($(#[$attrs:meta])* fn $func:ident($($id:ident : $tpe:ty,)*) -> $return_type:ty) => {
        extern "system" { 
            $(#[$attrs])*
            pub fn $func(
                $($id: $tpe,)*
            ) -> $return_type;
        }
    };
    ($(#[$attrs:meta])* fn $func:ident($($id:ident : $tpe:ty,)*)) => {
        extern "system" { 
            $(#[$attrs])*
            pub fn $func(
                $($id: $tpe,)*
            );
        }
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


// #[macro_export]
// macro_rules! UNION {
//     ($(#[$attrs:meta])* union $name:ident {
//         [$stype:ty; $ssize:expr],
//         $($variant:ident $variant_mut:ident: $ftype:ty,)+
//     }) => (
//         #[repr(C)] $(#[$attrs])*
//         pub struct $name([$stype; $ssize]);
//         impl Copy for $name {}
//         impl Clone for $name {
//             #[inline]
//             fn clone(&self) -> $name { *self }
//         }
//         #[cfg(feature = "impl-default")]
//         impl Default for $name {
//             #[inline]
//             fn default() -> $name { unsafe { $crate::_core::mem::zeroed() } }
//         }
//         impl $name {$(
//             #[inline]
//             pub unsafe fn $variant(&self) -> &$ftype {
//                 &*(self as *const _ as *const $ftype)
//             }
//             #[inline]
//             pub unsafe fn $variant_mut(&mut self) -> &mut $ftype {
//                 &mut *(self as *mut _ as *mut $ftype)
//             }
//         )+}
//     );
//     ($(#[$attrs:meta])* union $name:ident {
//         [$stype32:ty; $ssize32:expr] [$stype64:ty; $ssize64:expr],
//         $($variant:ident $variant_mut:ident: $ftype:ty,)+
//     }) => (
//         #[repr(C)] $(#[$attrs])* #[cfg(target_arch = "x86")]
//         pub struct $name([$stype32; $ssize32]);
//         #[repr(C)] $(#[$attrs])* #[cfg(target_arch = "x86_64")]
//         pub struct $name([$stype64; $ssize64]);
//         impl Copy for $name {}
//         impl Clone for $name {
//             #[inline]
//             fn clone(&self) -> $name { *self }
//         }
//         #[cfg(feature = "impl-default")]
//         impl Default for $name {
//             #[inline]
//             fn default() -> $name { unsafe { $crate::_core::mem::zeroed() } }
//         }
//         impl $name {$(
//             #[inline]
//             pub unsafe fn $variant(&self) -> &$ftype {
//                 &*(self as *const _ as *const $ftype)
//             }
//             #[inline]
//             pub unsafe fn $variant_mut(&mut self) -> &mut $ftype {
//                 &mut *(self as *mut _ as *mut $ftype)
//             }
//         )+}
//     );
// }

#[macro_export]
macro_rules! INTERFACE_BINDING {
    (interface $interface:ident ($vtbl:ident) {$(
        $(#[$($attrs:tt)*])* fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty,
    )+}) => (
        INTERFACE_BINDING!{@vtbl $interface $vtbl () $(
            $(#[$($attrs)*])* fn $method($($p: $t,)*) -> $rtr,
        )+}
        #[repr(C)]
        pub struct $interface {
            pub lpVtbl: *const $vtbl,
        }
        impl $interface {
            $(INTERFACE_BINDING!{@method $(#[$($attrs)*])* fn $method($($p: $t,)*) -> $rtr})+
        }
        INTERFACE_BINDING!{@deref $interface $pinterface}
    );
    (interface $interface:ident ($vtbl:ident) : $pinterface:ident ($pvtbl:ident) {
    }) => (
        INTERFACE_BINDING!{@vtbl $interface $vtbl (pub parent: $pvtbl,)}
        #[repr(C)]
        pub struct $interface {
            pub lpVtbl: *const $vtbl,
        }
        INTERFACE_BINDING!{@deref $interface $pinterface}
    );
    (interface $interface:ident ($vtbl:ident) : $pinterface:ident ($pvtbl:ident) {$(
        $(#[$($attrs:tt)*])* fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty,
    )+}) => (
        INTERFACE_BINDING!{@vtbl $interface $vtbl (pub parent: $pvtbl,) $(
            $(#[$($attrs)*])* fn $method($($p: $t,)*) -> $rtr,
        )+}
        #[repr(C)]
        pub struct $interface {
            pub lpVtbl: *const $vtbl,
        }
        impl $interface {
            $(INTERFACE_BINDING!{@method $(#[$($attrs)*])* fn $method($($p: $t,)*) -> $rtr})+
        }
        INTERFACE_BINDING!{@deref $interface $pinterface}
    );
    (@deref $interface:ident $pinterface:ident) => (
        impl super::core::Deref for $interface {
            type Target = $pinterface;
            #[inline]
            fn deref(&self) -> &$pinterface {
                unsafe { &*(self as *const $interface as *const $pinterface) }
            }
        }
    );
    (@method fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty) => (
        #[inline] pub unsafe fn $method(&self, $($p: $t,)*) -> $rtr {
            ((*self.lpVtbl).$method)(self as *const _ as *mut _, $($p,)*)
        }
    );
    (@method #[fixme] fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty) => (
        #[inline] pub unsafe fn $method(&self, $($p: $t,)*) -> $rtr {
            let mut ret = $crate::_core::mem::uninitialized();
            ((*self.lpVtbl).$method)(self as *const _ as *mut _, &mut ret, $($p,)*);
            ret
        }
    );
    (@vtbl $interface:ident $vtbl:ident ($($fields:tt)*)
        $(fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty,)*
    ) => (
        INTERFACE_BINDING!{@item #[repr(C)]
        pub struct $vtbl {
            $($fields)*
            $(pub $method: unsafe extern "system" fn(
                This: *mut $interface,
                $($p: $t,)*
            ) -> $rtr,)*
        }}
    );
    (@vtbl $interface:ident $vtbl:ident ($($fields:tt)*)
        fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty,
    $($tail:tt)*) => (
        INTERFACE_BINDING!{@vtbl $interface $vtbl (
            $($fields)*
            pub $method: unsafe extern "system" fn(
                This: *mut $interface,
                $($p: $t,)*
            ) -> $rtr,
        ) $($tail)*}
    );
    (@vtbl $interface:ident $vtbl:ident ($($fields:tt)*)
        #[fixme] fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty,
    $($tail:tt)*) => (
        INTERFACE_BINDING!{@vtbl $interface $vtbl (
            $($fields)*
            pub $method: unsafe extern "system" fn(
                This: *mut $interface,
                ret: *mut $rtr,
                $($p: $t,)*
            ) -> *mut $rtr,
        ) $($tail)*}
    );
    (@item $thing:item) => ($thing);
}

#[macro_export]
macro_rules! SIGNED_ENUM {
    {enum $name:ident { $($variant:ident = $value:expr,)+ }} => {
        pub type $name = i32;
        $(pub const $variant: $name = $value;)+
    };
    {enum $name:ident { $variant:ident = $value:expr, $($rest:tt)* }} => {
        pub type $name = i32;
        pub const $variant: $name = $value;
        SIGNED_ENUM!{@gen $name $variant, $($rest)*}
    };
    {enum $name:ident { $variant:ident, $($rest:tt)* }} => {
        SIGNED_ENUM!{enum $name { $variant = 0, $($rest)* }}
    };
    {@gen $name:ident $base:ident,} => {};
    {@gen $name:ident $base:ident, $variant:ident = $value:expr, $($rest:tt)*} => {
        pub const $variant: $name = $value;
        SIGNED_ENUM!{@gen $name $variant, $($rest)*}
    };
    {@gen $name:ident $base:ident, $variant:ident, $($rest:tt)*} => {
        pub const $variant: $name = $base + 1i32;
        SIGNED_ENUM!{@gen $name $variant, $($rest)*}
    };
}