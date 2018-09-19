// wrappers.rs - MIT License
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
use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

pub trait Sealed {}

//Trait for dealing with IUnknown pointers
pub trait RefCounted: Sealed {
    fn increment(&self) -> bool;
    fn decrement(&self) -> bool;
}

pub trait ComRelease: RefCounted {
    fn destruct(self) -> Result<(), ()>; //Need a possible error set
}

//Wrapper for heap-allocated memory, 
// Invariant: value is non-null and constructed
#[derive(Debug, PartialEq)]
pub struct PtrCtr<P> {
    inner: NonNull<P>
}

#[derive(Debug, PartialEq)]
pub enum WrapperErrors {
    IsNull
}

impl<P> PtrCtr<P> {
    #[must_use] 
    pub fn new_checked(p: *mut P) -> Result<PtrCtr<P>, WrapperErrors> {
        if let Some(nn) = NonNull::new(p) {
            Ok(PtrCtr{ inner: nn })
        } else {
            Err(WrapperErrors::IsNull) //Currently NonNull only returns None if p is null
        }
    }

    #[must_use]
    pub unsafe fn new_from(obj: NonNull<P>) -> PtrCtr<P> {
        PtrCtr { inner: obj }
    }

    #[must_use]
    pub fn as_mut(&mut self) -> *mut P {
        self.inner.as_ptr()
    }

    #[must_use]
    pub fn as_const(&self) -> *const P {
        self.inner.as_ptr()
    }

    pub fn into_mut(self) -> *mut P {
        let ptr = self.inner.as_ptr();
        mem::forget(self);
        ptr
    }

    pub fn into_ref<'p>(&self) -> RefCtr<'p, P> {
        RefCtr::new(self.inner.clone())
    }
}

pub struct RefCtr<'p, P>{
    inner: NonNull<P>, 
    _lifetime: PhantomData<&'p ()>,
} 

impl<'p, P> RefCtr<'p, P> {
    #[must_use]
    pub fn new(obj: NonNull<P>) -> RefCtr<'p, P> {
        RefCtr { inner: obj, _lifetime: PhantomData }
    }

    #[must_use]
    pub fn as_const(&self) -> *const P {
        self.inner.as_ptr()
    }
}

#[cfg(test)]
mod test {
    use std::ptr;
    use super::*;
    #[test]
    fn ctr_invariants() {
        let np: *mut i32 = ptr::null_mut();
        let r = PtrCtr::new_checked(np);
        assert_eq!(r, Err(WrapperErrors::IsNull));

        let p: *mut i32 = &mut 10 as *mut i32;
        let r = PtrCtr::new_checked(p);
        assert_eq!(r, Ok(PtrCtr{inner: NonNull::new(p).unwrap()})); 
    }
}