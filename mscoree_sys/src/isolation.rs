#![allow(dead_code, non_camel_case_types, non_snake_case)]
// isolation.rs - MIT License
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

use winapi::shared::basetsd::{SIZE_T};
use winapi::shared::minwindef::{BOOL, BYTE, DWORD, ULONG};
use winapi::shared::ntdef::{LPCWSTR, LPWSTR, WCHAR, ULONGLONG};
use winapi::shared::winerror::HRESULT;
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};


STRUCT!{struct _IDENTITY_ATTRIBUTE
{
    pszNamespace: LPCWSTR,
    pszName: LPCWSTR,
    pszValue: LPCWSTR,
}}
pub type IDENTITY_ATTRIBUTE = _IDENTITY_ATTRIBUTE;
pub type PIDENTITY_ATTRIBUTE = *mut _IDENTITY_ATTRIBUTE;

RIDL!{#[uuid(0x6eaf5ace, 0x7917, 0x4f3c, 0xb1, 0x29, 0xe0, 0x46, 0xa9, 0x70, 0x47, 0x66)]
interface IReferenceIdentity(IReferenceIdentityVtbl): IUnknown(IUnknownVtbl){
    fn GetAttribute(
        pszNamespace: LPCWSTR, 
        pszName: LPCWSTR, 
        ppszValue: *mut LPCWSTR, 
    ) -> HRESULT, 
    fn SetAttribute(
        pszNamespace: LPCWSTR, 
        pszName: LPCWSTR, 
        pszValue: LPCWSTR, 
    ) -> HRESULT, 
    fn EnumAttributes(
        ppIEnumIDENTITY_ATTRIBUTE: *mut *mut IEnumIDENTITY_ATTRIBUTE,
    ) -> HRESULT, 
    fn Clone_(
        cDeltas: SIZE_T, 
        rgDeltas: *const IDENTITY_ATTRIBUTE,
        ppIReferenceIdentity: *mut *mut IReferenceIdentity,
    ) -> HRESULT, 
}}

RIDL!{#[uuid(0x587bf538, 0x4d90, 0x4a3c, 0x9e, 0xf1, 0x58, 0xa2, 0x00, 0xa8, 0xa9, 0xe7)]
interface IDefinitionIdentity(IDefinitionIdentityVtbl): IUnknown(IUnknownVtbl){
    fn GetAttribute(
        pszNamespace: LPCWSTR, 
        pszName: LPCWSTR, 
        ppszValue: *mut LPWSTR, 
    ) -> HRESULT,
    fn SetAttribute(
        pszNamespace: LPCWSTR, 
        pszName: LPCWSTR, 
        pszValue: LPCWSTR, 
    ) -> HRESULT, 
    fn EnumAttributes(
        ppIEAIA: *mut *mut IEnumIDENTITY_ATTRIBUTE, 
    ) -> HRESULT, 
    fn Clone_(
        cDeltas: SIZE_T, 
        rgDeltas: *const IDENTITY_ATTRIBUTE,
        ppIDefinitionIdentity: *mut *mut IDefinitionIdentity,
    ) -> HRESULT, 
}}

STRUCT!{struct _IDENTITY_ATTRIBUTE_BLOB
{
    ofsNamespace: DWORD,
    ofsName: DWORD,
    ofsValue: DWORD,
}}
pub type IDENTITY_ATTRIBUTE_BLOB = _IDENTITY_ATTRIBUTE;
pub type PIDENTITY_ATTRIBUTE_BLOB = *mut _IDENTITY_ATTRIBUTE;

RIDL!{#[uuid(0x9cdaae75, 0x246e, 0x4b00, 0xa2, 0x6d, 0xb9, 0xae, 0xc1, 0x37, 0xa3, 0xeb)]
interface IEnumIDENTITY_ATTRIBUTE(IEnumIDENTITY_ATTRIBUTEVtbl): IUnknown(IUnknownVtbl){
    fn Next(
        celt: ULONG, 
        rgAttributes: *mut IDENTITY_ATTRIBUTE, 
        pceltWritten: *mut ULONG,
    ) -> HRESULT, 
    fn CurrentIntoBuffer(
        cbAvailable: SIZE_T, 
        pbData: *mut BYTE, 
        pcbUsed: *mut SIZE_T, 
    ) -> HRESULT, 
    fn Skip(
        celt: ULONG, 
    ) -> HRESULT, 
    fn Reset() -> HRESULT, 
    fn Clone_(
        ppIEnumIDENTITY_ATTRIBUTE: *mut *mut IEnumIDENTITY_ATTRIBUTE,
    ) -> HRESULT, 
}}

RIDL!{#[uuid(0xf3549d9c, 0xfc73, 0x4793, 0x9c, 0x00, 0x1c, 0xd2, 0x04, 0x25, 0x4c, 0x0c)]
interface IEnumDefinitionIdentity(IEnumDefinitionIdentityVtbl): IUnknown(IUnknownVtbl){
    fn Next(
        celt: ULONG, 
        rgpIDefinitionIdentity: *mut IDefinitionIdentity, 
        pceltWritten: *mut ULONG, 
    ) -> HRESULT, 
    fn Skip(
        celt: ULONG,
    ) -> HRESULT, 
    fn Reset() -> HRESULT, 
    fn Clone_(
        ppIEnumDefinitionIdentity: *mut *mut IEnumDefinitionIdentity,
    ) -> HRESULT, 
}}

RIDL!{#[uuid(0xb30352cf, 0x23da, 0x4577, 0x9b, 0x3f, 0xb4, 0xe6, 0x57, 0x3b, 0xe5, 0x3b)]
interface IEnumReferenceIdentity(IEnumReferenceIdentityVtbl): IUnknown(IUnknownVtbl){
    fn Next(
        celt: ULONG, 
        prgpIReferenceIdentity: *mut *mut IReferenceIdentity,
        pceltWritten: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT, 
    fn Reset() -> HRESULT,
    fn Clone_(
        ppIEnumReferenceIdentity: *mut *mut IEnumReferenceIdentity,
    ) -> HRESULT, 
}}

RIDL!{#[uuid(0xd91e12d8, 0x98ed, 0x47fa, 0x99, 0x36, 0x39, 0x42, 0x12, 0x83, 0xd5, 0x9b)]
interface IDefinitionAppId(IDefinitionAppIdVtbl): IUnknown(IUnknownVtbl){
    fn get_SubscriptionId(
        ppszSubscription: *mut LPWSTR, 
    ) -> HRESULT, 
    fn put_SubscriptionId(
        pszSubscription: LPCWSTR, 
    ) -> HRESULT, 
    fn get_Codebase(
        ppszCodebase: *mut LPWSTR, 
    ) -> HRESULT, 
    fn put_Codebase(
        pszCodebase: LPCWSTR, 
    ) -> HRESULT, 
    fn EnumAppPath(
        ppIEnumDefinitionIdentity: *mut *mut IEnumDefinitionIdentity, 
    ) -> HRESULT,
    fn SetAppPath(
        cIDefinitionIdentity: ULONG, 
        rgIDefinitionIdentity: *mut *mut IDefinitionIdentity,
    ) -> HRESULT, 
}}
//IReferenceAppId
RIDL!{#[uuid(0x054f0bef, 0x9e45, 0x4363, 0x8f, 0x5a, 0x2f, 0x8e, 0x14, 0x2d, 0x9a, 0x3b)]
interface IReferenceAppId(IReferenceAppIdVtbl): IUnknown(IUnknownVtbl){
    fn get_SubscriptionId(
        ppszSubscription: *mut LPWSTR, 
    ) -> HRESULT, 
    fn put_SubscriptionId(
        pszSubscription: LPCWSTR, 
    ) -> HRESULT, 
    fn get_Codebase(
        ppszCodebase: *mut LPWSTR, 
    ) -> HRESULT, 
    fn put_Codebase(
        pszCodebase: LPCWSTR, 
    ) -> HRESULT, 
    fn EnumAppPath(
        ppIReferenceAppId: *mut *mut IEnumReferenceIdentity, 
    ) -> HRESULT,
}}

ENUM!{enum _TEXT_TO_DEFINITION_IDENTITY_FLAGS {	
    TEXT_TO_DEFINITION_IDENTITY_FLAG_ALLOW_UNKNOWN_ATTRIBUTES_IN_NULL_NAMESPACE	= 0x1,
}}
ENUM!{enum _TEXT_TO_REFERENCE_IDENTITY_FLAGS {	
    TEXT_TO_REFERENCE_IDENTITY_FLAG_ALLOW_UNKNOWN_ATTRIBUTES_IN_NULL_NAMESPACE	= 0x1,
}}
ENUM!{enum _DEFINITION_IDENTITY_TO_TEXT_FLAGS{	
    DEFINITION_IDENTITY_TO_TEXT_FLAG_CANONICAL	= 0x1,
}}
ENUM!{enum _REFERENCE_IDENTITY_TO_TEXT_FLAGS {	
    REFERENCE_IDENTITY_TO_TEXT_FLAG_CANONICAL	= 0x1,
}}
ENUM!{enum _IIDENTITYAUTHORITY_DOES_DEFINITION_MATCH_REFERENCE_FLAGS{	
    IIDENTITYAUTHORITY_DOES_DEFINITION_MATCH_REFERENCE_FLAG_EXACT_MATCH_REQUIRED	= 0x1,
}}
ENUM!{enum _IIDENTITYAUTHORITY_DOES_TEXTUAL_DEFINITION_MATCH_TEXTUAL_REFERENCE_FLAGS{
	IIDENTITYAUTHORITY_DOES_TEXTUAL_DEFINITION_MATCH_TEXTUAL_REFERENCE_FLAG_EXACT_MATCH_REQUIRED	= 0x1,
}}

//IIdentityAuthority
RIDL!{#[uuid(0x261a6983, 0xc35d, 0x4d0d, 0xaa, 0x5b, 0x78, 0x67, 0x25, 0x9e, 0x77, 0xbc)]
interface IIdentityAuthority(IIdentityAuthorityVtbl): IUnknown(IUnknownVtbl){
    fn TextToDefinition(
        dwFlags: DWORD, 
        pszIdentity: LPCWSTR, 
        ppIDefinitionIdentity: *mut *mut IDefinitionIdentity,
    ) -> HRESULT, 
    fn TextToReference(
        dwFlags: DWORD, 
        pszIdentity: LPCWSTR, 
        ppIReferenceIdentity: *mut *mut IReferenceIdentity,
    ) -> HRESULT,  
    fn DefinitionToText(
        dwFlags: DWORD, 
        pIDefinitionIdentity: *mut IDefinitionIdentityVtbl, 
        ppszFormattedIdentity: *mut LPWSTR, 
    ) -> HRESULT, 
    fn DefinitionToTextBuffer(
        dwFlags: DWORD, 
        pIDefinitionIdentity: *mut IDefinitionIdentity, 
        cchBufferSize: ULONG, 
        wchBuffer: *mut WCHAR, 
        pcchBufferRequired: *mut ULONG, 
    ) -> HRESULT, 
    fn ReferenceToText(
        dwFlags: DWORD,
        pIReferenceIdentity: *mut IReferenceIdentity, 
        ppszFormattedIdentity: *mut LPWSTR, 
    ) -> HRESULT,
    fn ReferenceToTextBuffer(
        dwFlags: DWORD, 
        pIReferenceIdentity: *mut IReferenceIdentity, 
        cchBufferSize: ULONG, 
        wchBuffer: *mut WCHAR, 
        pcchBufferRequired: *mut ULONG, 
    ) -> HRESULT, 
    fn AreDefinitionsEqual(
        dwFlags: DWORD, 
        pDefinition1: *mut IDefinitionIdentity, 
        pDefinition2: *mut IDefinitionIdentity, 
        pfEqual: *mut BOOL,
    ) -> HRESULT, 
    fn AreReferencesEqual(
        dwFlags: DWORD, 
        pReference1: *mut IReferenceIdentity, 
        pReference2: *mut IReferenceIdentity, 
        pfEqual: *mut BOOL,
    ) -> HRESULT, 
    fn AreTextualDefinitionsEqual(
        dwFlags: DWORD, 
        pszIdentityLeft: LPCWSTR, 
        pszIdentityRight: LPCWSTR, 
        pfEqual: *mut BOOL,
    ) -> HRESULT, 
    fn AreTextualReferencesEqual(
        dwFlags: DWORD, 
        pszIdentityLeft: LPCWSTR, 
        pszIdentityRight: LPCWSTR, 
        pfEqual: *mut BOOL,
    ) -> HRESULT,
    fn DoesDefinitionMatchReference(
        dwFlags: DWORD, 
        pIDefinitionIdentity: *mut IDefinitionIdentity, 
        pIReferenceIdentity: *mut IReferenceIdentity, 
        pfMatches: *mut BOOL,
    ) -> HRESULT,
    fn DoesTextualDefinitionMatchTextualReference(
        dwFlags: DWORD, 
        pszDefinition: LPCWSTR, 
        pszReference: LPCWSTR, 
        pfMatches: *mut BOOL,
    ) -> HRESULT,
    fn HashReference(
        dwFlags: DWORD, 
        pIReferenceIdentity: *mut IReferenceIdentity,
        pullPseudoKey: *mut ULONGLONG,
    ) -> HRESULT,
    fn HashDefinition(
        dwFlags: DWORD, 
        pIDefinitionIdentity: *mut IDefinitionIdentity,
        pullPseudoKey: *mut ULONGLONG,
    ) -> HRESULT,
}}
//2 more enums
//IAppIdAuthority
/*/* [local] */ HRESULT __stdcall GetAppIdAuthority( 
    /* [out] */ IAppIdAuthority **ppIAppIdAuthority);

/* [local] */ HRESULT __stdcall GetIdentityAuthority( 
    /* [out] */ IIdentityAuthority **ppIIdentityAuthority);
*/
