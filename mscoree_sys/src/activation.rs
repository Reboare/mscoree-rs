#![allow(dead_code, non_upper_case_globals, non_camel_case_types, non_snake_case)]
use winapi::shared::winerror::HRESULT;

use crate::inspectable::{IInspectable, IInspectableVtbl};

RIDL!{#[uuid(0x00000035, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IActivationFactory(IActivationFactoryVtbl): IInspectable(IInspectableVtbl){
    fn ActivateInstance(
        instance: *mut *mut IInspectable,
    ) -> HRESULT,
}}
