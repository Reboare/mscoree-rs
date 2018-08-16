#![allow(dead_code, non_upper_case_globals, non_camel_case_types, non_snake_case)]
use winapi::shared::guiddef::IID;
use winapi::shared::minwindef::ULONG;
use winapi::shared::winerror::HRESULT;

use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::winrt::hstring::HSTRING;

ENUM!{enum TrustLevel
{
    BaseTrust	= 0,
    PartialTrust	= ( BaseTrust + 1 ) ,
    FullTrust	= ( PartialTrust + 1 ), 
}}

RIDL!{#[uuid(0xAF86E2E0, 0xB12D, 0x4c6a, 0x9C, 0x5A, 0xD7, 0xAA, 0x65, 0x10, 0x1E, 0x90)]
interface IInspectable(IInspectableVtbl): IUnknown(IUnknownVtbl){
    fn GetIids(
        iidCount: *mut ULONG, 
        iids: *mut *mut IID, 
    ) -> HRESULT,
    fn GetRuntimeClassName(
        className: *mut HSTRING, 
    ) -> HRESULT, 
    fn GetTrustLevel(
        trustLevel: *mut TrustLevel, 
    ) -> HRESULT,
}}