#[macro_use] extern crate winapi;
extern crate mscorlib_sys;

#[macro_use] pub mod macros;

pub mod activation;
pub mod inspectable;
pub mod gchost;
pub mod metahost;
pub mod mscoree;