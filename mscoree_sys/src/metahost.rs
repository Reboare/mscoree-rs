#![allow(dead_code, non_snake_case, non_upper_case_globals, non_camel_case_types)]
// metahost.rs - MIT License
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


use winapi::ctypes::c_void;
use winapi::shared::basetsd::ULONG64;
use winapi::shared::guiddef::{CLSID, REFCLSID, REFIID};
use winapi::shared::minwindef::{BOOL, BYTE, DWORD, HMODULE, LPVOID, UINT, ULONG, WORD};
use winapi::shared::ntdef::{HANDLE, LONG, LPCSTR, LPCWSTR, LPWSTR, WCHAR};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypesbase::BOOLEAN;

use winapi::um::objidlbase::{IEnumUnknown, IStream};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

extern "system" {
    pub fn CLRCreateInstance(
        cldsid: REFCLSID,
        riid: REFIID, 
        ppInterface: *mut LPVOID
    ) -> HRESULT;
}

DEFINE_GUID!(CLSID_CLRStrongName, 0xB79B0ACD, 0xF5CD, 0x409b, 0xB5, 0xA5, 0xA1, 0x62, 0x44, 0x61, 0x0B, 0x92);
DEFINE_GUID!(IID_ICLRMetaHost, 0xD332DB9E, 0xB9B3, 0x4125, 0x82, 0x07, 0xA1, 0x48, 0x84, 0xF5, 0x32, 0x16);
DEFINE_GUID!(CLSID_CLRMetaHost, 0x9280188d, 0xe8e, 0x4867, 0xb3, 0xc, 0x7f, 0xa8, 0x38, 0x84, 0xe8, 0xde);
DEFINE_GUID!(IID_ICLRMetaHostPolicy, 0xE2190695, 0x77B2, 0x492e, 0x8E, 0x14, 0xC4, 0xB3, 0xA7, 0xFD, 0xD5, 0x93);
DEFINE_GUID!(CLSID_CLRMetaHostPolicy, 0x2ebcd49a, 0x1b47, 0x4a61, 0xb1, 0x3a, 0x4a, 0x3, 0x70, 0x1e, 0x59, 0x4b);
DEFINE_GUID!(IID_ICLRDebugging, 0xd28f3c5a, 0x9634, 0x4206, 0xa5, 0x9, 0x47, 0x75, 0x52, 0xee, 0xfb, 0x10);
DEFINE_GUID!(CLSID_CLRDebugging, 0xbacc578d, 0xfbdd, 0x48a4, 0x96, 0x9f, 0x2, 0xd9, 0x32, 0xb7, 0x46, 0x34);
DEFINE_GUID!(IID_ICLRRuntimeInfo, 0xBD39D1D2, 0xBA2F, 0x486a, 0x89, 0xB0, 0xB4, 0xB0, 0xCB, 0x46, 0x68, 0x91);
DEFINE_GUID!(IID_ICLRStrongName, 0x9FD93CCF, 0x3280, 0x4391, 0xB3, 0xA9, 0x96, 0xE1, 0xCD, 0xE7, 0x7C, 0x8D);
DEFINE_GUID!(IID_ICLRStrongName2, 0xC22ED5C5, 0x4B59, 0x4975, 0x90, 0xEB, 0x85, 0xEA, 0x55, 0xC0, 0x06, 0x9B);
DEFINE_GUID!(IID_ICLRStrongName3, 0x22c7089b, 0xbbd3, 0x414a, 0xb6, 0x98, 0x21, 0x0f, 0x26, 0x3f, 0x1f, 0xed);
DEFINE_GUID!(CLSID_CLRDebuggingLegacy, 0xDF8395B5, 0xA4BA, 0x450b, 0xA7, 0x7C, 0xA9, 0xA4, 0x77, 0x62, 0xC5, 0x20);
DEFINE_GUID!(CLSID_CLRProfiling, 0xbd097ed8, 0x733e, 0x43fe, 0x8e, 0xd7, 0xa9, 0x5f, 0xf9, 0xa8, 0x44, 0x8c);
DEFINE_GUID!(IID_ICLRProfiling, 0xb349abe3, 0xb56f, 0x4689, 0xbf, 0xcd, 0x76, 0xbf, 0x39, 0xd8, 0x88, 0xea);
DEFINE_GUID!(IID_ICLRDebuggingLibraryProvider, 0x3151c08d, 0x4d09, 0x4f9b, 0x88, 0x38, 0x28, 0x80, 0xbf, 0x18, 0xfe, 0x51);
DEFINE_GUID!(IID_ICLRDebuggingLibraryProvider2, 0xE04E2FF1, 0xDCFD, 0x45D5, 0xBC, 0xD1, 0x16, 0xFF, 0xF2, 0xFA, 0xF7, 0xBA);

FUNC_PTR!{CLRCreateInstanceFnPtr(rclsid: REFCLSID, riid: REFIID, pvoid: *mut LPVOID) -> HRESULT}
FUNC_PTR!{CreateInterfaceFnPtr(rclsid: REFCLSID, riid: REFIID, pvoid: *mut LPVOID) -> HRESULT}
FUNC_PTR!{CallbackThreadSetFnPtr() -> HRESULT}
FUNC_PTR!{CallbackThreadUnsetFnPtr() -> HRESULT}
FUNC_PTR!{RuntimeLoadedCallbackFnPtr(
    pInfo: *mut ICLRRuntimeInfo, 
    pfnCallbackThreadSet: CallbackThreadSetFnPtr, 
    pfnCallbackThreadUnset: CallbackThreadUnsetFnPtr)}

RIDL!{#[uuid(0xD332DB9E, 0xB9B3, 0x4125, 0x82, 0x07, 0xA1, 0x48, 0x84, 0xF5, 0x32, 0x16)]
interface ICLRMetaHost(ICLRMetaHostVtbl): IUnknown(IUnknownVtbl){
    fn GetRuntime(
        pwzVersion: LPCWSTR, 
        riid: REFIID, 
        ppRuntime: *mut LPVOID,
    ) -> HRESULT,
    fn GetVersionFromFile(
        pwzFilePath: LPCWSTR, 
        pwzBuffer: LPWSTR, 
        pcchBuffer: DWORD, 
    ) -> HRESULT, 
    fn EnumerateInstalledRuntimes(
        ppEnumerator: *mut *mut IEnumUnknown,
    ) -> HRESULT,
    fn EnumerateLoadedRuntimes(
        hndProcess: HANDLE, 
        ppEnumerator: *mut *mut IEnumUnknown,
    ) -> HRESULT,
    fn RequestRuntimeLoadedNotification (
        pCallbackFunction: RuntimeLoadedCallbackFnPtr,
    ) -> HRESULT, 
    fn QueryLegacyV2RuntimeBinding(
        riid: REFIID, 
        ppUnk: *mut LPVOID, 
    ) -> HRESULT, 
    fn ExitProcess(
        iExitCode: i32,
    ) -> HRESULT,
}}

ENUM!{enum __MIDL___MIDL_itf_metahost_0000_0001_0001 {
    METAHOST_POLICY_HIGHCOMPAT	= 0,
    METAHOST_POLICY_APPLY_UPGRADE_POLICY	= 0x8,
    METAHOST_POLICY_EMULATE_EXE_LAUNCH	= 0x10,
    METAHOST_POLICY_SHOW_ERROR_DIALOG	= 0x20,
    METAHOST_POLICY_USE_PROCESS_IMAGE_PATH	= 0x40,
    METAHOST_POLICY_ENSURE_SKU_SUPPORTED	= 0x80,
    METAHOST_POLICY_IGNORE_ERROR_MODE	= 0x1000,
}}
pub type METAHOST_POLICY_FLAGS = __MIDL___MIDL_itf_metahost_0000_0001_0001;

ENUM!{enum __MIDL___MIDL_itf_metahost_0000_0001_0002
{
    METAHOST_CONFIG_FLAGS_LEGACY_V2_ACTIVATION_POLICY_UNSET	= 0,
    METAHOST_CONFIG_FLAGS_LEGACY_V2_ACTIVATION_POLICY_TRUE	= 0x1,
    METAHOST_CONFIG_FLAGS_LEGACY_V2_ACTIVATION_POLICY_FALSE	= 0x2,
    METAHOST_CONFIG_FLAGS_LEGACY_V2_ACTIVATION_POLICY_MASK	= 0x3,
}}
pub type METAHOST_CONFIG_FLAGS = __MIDL___MIDL_itf_metahost_0000_0001_0002;

RIDL!{#[uuid(0xE2190695, 0x77B2, 0x492e, 0x8E, 0x14, 0xC4, 0xB3, 0xA7, 0xFD, 0xD5, 0x93)]
interface ICLRMetaHostPolicy(ICLRMetaHostPolicyVtbl): IUnknown(IUnknownVtbl){
    fn GetRequestedRuntime(
        dwPolicyFlags: METAHOST_POLICY_FLAGS, 
        pwzBinary: LPCWSTR, 
        pCfgStream: *mut IStream,
        pwzVersion: LPWSTR, 
        pcchVersion: *mut DWORD, 
        pwzImageVersion: LPWSTR, 
        pcchImageVersion: *mut DWORD, 
        pdwConfigFlags: *mut DWORD, 
        riid: REFIID,
        ppRuntime: *mut LPVOID,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xB349ABE3, 0xB56F, 0x4689, 0xBF, 0xCD, 0x76, 0xBF, 0x39, 0xD8, 0x88, 0xEA)]
interface ICLRProfiling(ICLRProfilingVtbl): IUnknown(IUnknownVtbl){
    fn AttachProfiler(
        dwProfileeProcessID: DWORD, 
        dwMillisecondsMax: DWORD, 
        pCldsidProfiler: *const CLSID, 
        wszProfilerPath: LPCWSTR, 
        pvClientData: *mut c_void, 
        cbClientData: UINT, 
    ) -> HRESULT, 
}}

STRUCT!{struct _CLR_DEBUGGING_VERSION {
    wStructVersion: WORD, 
    wMajor: WORD, 
    wMinor: WORD, 
    wBuild: WORD, 
    wRevision: WORD, 
}}
pub type CLR_DEBUGGING_VERSION = _CLR_DEBUGGING_VERSION;

ENUM!{enum __MIDL___MIDL_itf_metahost_0000_0003_0001 {
    CLR_DEBUGGING_MANAGED_EVENT_PENDING	= 1,
    CLR_DEBUGGING_MANAGED_EVENT_DEBUGGER_LAUNCH	= 2,
}}
pub type CLR_DEBUGGING_PROCESS_FLAGS = __MIDL___MIDL_itf_metahost_0000_0003_0001;

RIDL!{#[uuid(0x3151C08D, 0x4D09, 0x4f9b, 0x88, 0x38, 0x28, 0x80, 0xBF, 0x18, 0xFE, 0x51)]
interface ICLRDebuggingLibraryProvider(ICLRDebuggingLibraryProviderVtbl): IUnknown(IUnknownVtbl){
    fn ProvideLibrary(    
        pwszFileName: *const WCHAR, 
        dwTimestamp: DWORD, 
        dwSizeOfImage: DWORD, 
        phModule: *mut HMODULE,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xE04E2FF1, 0xDCFD, 0x45D5, 0xBC, 0xD1, 0x16, 0xFF, 0xF2, 0xFA, 0xF7, 0xBA)]
interface ICLRDebuggingLibraryProvider2(ICLRDebuggingLibraryProvider2Vtbl): ICLRDebuggingLibraryProvider(ICLRDebuggingLibraryProviderVtbl){
    fn ProvideLibrary2(    
        pwszFileName: *const WCHAR, 
        dwTimestamp: DWORD, 
        dwSizeOfImage: DWORD, 
        phModule: *mut HMODULE,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xD28F3C5A, 0x9634, 0x4206, 0xA5, 0x09, 0x47, 0x75, 0x52, 0xEE, 0xFB, 0x10)]
interface ICLRDebugging(ICLRDebuggingVtbl): IUnknown(IUnknownVtbl){
    fn OpenVirtualProcess(
        moduleBaseAdress: ULONG64, 
        pDataTarget: *mut IUnknown, 
        pLibraryProvider: *mut ICLRDebuggingLibraryProvider, 
        pMaxDebuggerSupportedVersion: *mut CLR_DEBUGGING_VERSION, 
        riidProcess: REFIID, 
        ppProcess: *mut *mut IUnknown, 
        pVersion: *mut CLR_DEBUGGING_VERSION, 
        pdwFlags: *mut CLR_DEBUGGING_PROCESS_FLAGS, 
    ) -> HRESULT,
    fn CanUnloadNow(
        hModule: HMODULE,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xBD39D1D2, 0xBA2F, 0x486a, 0x89, 0xB0, 0xB4, 0xB0, 0xCB, 0x46, 0x68, 0x91)]
interface ICLRRuntimeInfo(ICLRRuntimeInfoVtbl): IUnknown(IUnknownVtbl){
    fn GetVersionString(
        pwzBuffer: LPWSTR, 
        pcchBuffer: *mut DWORD,
    ) -> HRESULT, 
    fn GetRuntimeDirectory(
        pwzBuffer: LPWSTR, 
        pcchBuffer: *mut DWORD,
    ) -> HRESULT, 
    fn IsLoaded(
        hndProcess: HANDLE, 
        pbLoaded: *mut BOOL, 
    ) -> HRESULT, 
    fn LoadErrorString(
        iResourceID: UINT, 
        pwzBuffer: LPWSTR, 
        pcchBuffer: *mut DWORD, 
        iLocaleID: LONG,
    ) -> HRESULT,
    fn LoadLibrary(
        pwzDllName: LPCWSTR, 
        ppProc: *mut LPVOID,
    ) -> HRESULT,
    fn GetProcAddress(
        pszProcName: LPCSTR, 
        ppProc: *mut LPVOID, 
    ) -> HRESULT,
    fn GetInterface(
        rclsid: REFCLSID, 
        riid: REFIID, 
        ppUnk: *mut LPVOID, 
    ) -> HRESULT,
    fn IsLoadable(
        pbLoadable: *mut BOOL,
    ) -> HRESULT,
    fn SetDefaultStartupFlags(
        dwStartupFlags: DWORD, 
        pwzHostConfigFile: LPCWSTR, 
    ) -> HRESULT,
    fn GetDefaultStartupFlags(
        pdwStartupFlags: *mut DWORD, 
        pwzHostConfigFile: LPWSTR, 
        pcchHostConfigFile: *mut DWORD, 
    ) -> HRESULT, 
    fn BindAsLegacyV2Runtime() -> HRESULT, 
    fn IsStarted(
        pbStarted: *mut BOOL, 
        pdwStartupFlags: *mut DWORD,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9FD93CCF, 0x3280, 0x4391, 0xB3, 0xA9, 0x96, 0xE1, 0xCD, 0xE7, 0x7C, 0x8D)]
interface ICLRStrongName(ICLRStrongNameVtbl): IUnknown(IUnknownVtbl){
    fn GetHashFromAssemblyFile(
        pszFilePath: LPCSTR, 
        piHashAlg: *mut UINT, 
        pbHash: *mut BYTE, 
        cchHash: DWORD, 
        pchHash: *mut DWORD,
    ) -> HRESULT,
    fn GetHashFromAssemblyFileW(
        pwzFilePath: LPCWSTR, 
        piHashAlg: *mut UINT, 
        pbHash: *mut BYTE, 
        cchHash: DWORD, 
        pchHash: *mut DWORD, 
    ) -> HRESULT,
    fn GetHashFromBlob(
        pbBlob: *mut BYTE, 
        cchBlob: DWORD, 
        piHashAlg: *mut UINT, 
        cchHash: DWORD, 
        pchHash: *mut DWORD, 
    ) -> HRESULT, 
    fn GetHashFromFile(
        pszFilePath: LPCSTR, 
        piHashAlg: *mut UINT, 
        pbHash: *mut BYTE, 
        cchHash: DWORD, 
        pchHash: *mut DWORD, 
    ) -> HRESULT,
    fn GetHashFromFileW(
        pszFilePath: LPCWSTR, 
        piHashAlg: *mut UINT, 
        pbHash: *mut BYTE, 
        cchHash: DWORD, 
        pchHash: *mut DWORD, 
    ) -> HRESULT,
    fn GetHashFromHandle(
        hFile: HANDLE, 
        piHashAlg: *mut UINT, 
        pbHash: *mut BYTE, 
        cchHash: DWORD, 
        pchHash: *mut DWORD, 
    ) -> HRESULT,
    fn StrongNameCompareAssemblies(
        pwzAssembly1: LPCWSTR, 
        pwzAssembly2: LPCWSTR, 
        pdwResult: *mut DWORD, 
    ) -> HRESULT, 
    fn StrongNameFreeBuffer(
        pbMemory: *mut BYTE, 
    ) -> HRESULT, 
    fn StrongNameGetBlob(
        pwzFilePath: LPCWSTR, 
        pbBlob: *mut BYTE, 
        pcbBlob: *mut DWORD, 
    ) -> HRESULT, 
    fn StrongNameGetBlobFromImage(
        pbBase: *mut BYTE, 
        dwLength: DWORD, 
        pbBlob: *mut BYTE, 
        pcbBlob: *mut DWORD, 
    ) -> HRESULT,
    fn StrongNameGetPublicKey(
        pwzKeyContainer: LPCWSTR, 
        pbKeyBlob: *mut BYTE, 
        cbKeyBlob: ULONG, 
        ppbPublicKeyBlob: *mut *mut BYTE, 
        pcbPublicKeyBlob: *mut ULONG,
    ) -> HRESULT,
    fn StrongNameHashSize(
        ulHashAlg: ULONG, 
        pcbSize: *mut DWORD,
    ) -> HRESULT,
    fn StrongNameKeyDelete(
        pwzKeyContainer: LPCWSTR, 
    ) -> HRESULT,
    fn StrongNameKeyGen(
        pwzKeyContainer: LPCWSTR, 
        dwFlags: DWORD, 
        ppbKeyBlob: *mut *mut BYTE, 
        pcbKeyBlob: *mut ULONG,  
    ) -> HRESULT, 
    fn StrongNameKeyGenEx(
        pwzKeyContainer: LPCWSTR, 
        dwFlags: DWORD, 
        dwKeySize: DWORD,
        ppbKeyBlob: *mut *mut BYTE, 
        pcbKeyBlob: *mut ULONG,  
    ) -> HRESULT, 
    fn StrongNameKeyInstall(
        pwzKeyContainer: LPCWSTR, 
        pbKeyBlob: *mut BYTE, 
        cbKeyBlob: ULONG,
    ) -> HRESULT,
    fn StrongNameSignatureGeneration(
        pwzFilePath: LPCWSTR, 
        pwzKeyContainer: LPCWSTR, 
        pbKeyBlob: *mut BYTE, 
        cbKeyBlob: ULONG, 
        ppbSignatureBlob: *mut *mut BYTE, 
        pcbSignatureBlob: *mut ULONG,
    ) -> HRESULT,
    fn StrongNameSignatureGenerationEx(
        pwzFilePath: LPCWSTR, 
        pwzKeyContainer: LPCWSTR, 
        pbKeyBlob: *mut BYTE, 
        cbKeyBlob: ULONG, 
        ppbSignatureBlob: *mut *mut BYTE, 
        pcbSignatureBlob: *mut ULONG,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn StrongNameSignatureSize(
        pbPublicKeyBlob: *mut BYTE, 
        cbPublicKeyBlob: ULONG, 
        pcbSize: *mut DWORD,
    ) -> HRESULT,
    fn StrongNameSignatureVerification(
        pwzFilePath: LPCWSTR, 
        dwInFlags: DWORD, 
        pdwOutFlags: *mut DWORD,
    ) -> HRESULT,
    fn StrongNameSignatureVerificationEx(
        pwzFilePath: LPCWSTR, 
        fForceVerification: BOOLEAN, 
        pfWasVerified: *mut BOOLEAN,
    ) -> HRESULT,
    fn StrongNameVerificationFromImage(
        pbBase: *mut BYTE, 
        dwLength: DWORD, 
        dwInFlags: DWORD, 
        pdwOutFlags: *mut DWORD, 
    ) -> HRESULT, 
    fn StrongNameTokenFromAssembly(
        pwzFilePath: LPCWSTR, 
        ppbStrongNameToken: *mut *mut BYTE, 
        pcbStrongNameToken: *mut ULONG, 
    ) -> HRESULT,
    fn StrongNameTokenFromAssemblyEx(
        pwzFilePath: LPCWSTR,
        ppbStrongNameToken: *mut *mut BYTE, 
        pcbStrongNameToken: *mut ULONG, 
        ppbPublicKeyBlob: *mut *mut BYTE, 
        pcbPublicKeyBlob: *mut ULONG,
    ) -> HRESULT,
    fn StrongNameTokenFromPublicKey(
        pbPublicKeyBlob: *mut BYTE, 
        cbPublicKeyBlob: ULONG, 
        ppbStrongNameToken: *mut *mut BYTE, 
        pcbStrongNameToken: *mut ULONG,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xC22ED5C5, 0x4B59, 0x4975, 0x90, 0xEB, 0x85, 0xEA, 0x55, 0xC0, 0x06, 0x9B)]
interface ICLRStrongName2(ICLRStrongName2Vtbl): IUnknown(IUnknownVtbl){
    fn StrongNameGetPublicKeyEx(
        pwzKeyContainer: LPCWSTR, 
        pbKeyBlob: *mut BYTE, 
        cbKeyBlob: ULONG, 
        ppbPublicKeyBlob: *mut *mut BYTE, 
        pcbPublicKeyBlob: *mut ULONG, 
        ulHashAlg: ULONG, 
        uReserved: ULONG, 
    ) -> HRESULT,
    fn StrongNameSignatureVerificationEx2(
        wszFilePath: LPCWSTR, 
        fForceVerification: BOOLEAN, 
        pbEcmaPublicKey: *mut BYTE, 
        cbEcmaPublicKey: DWORD, 
        pfWasVerified: *mut BOOLEAN,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x22c7089b, 0xbbd3, 0x414a, 0xb6, 0x98, 0x21, 0x0f, 0x26, 0x3f, 0x1f, 0xed)]
interface ICLRStrongName3(ICLRStrongName3Vtbl): IUnknown(IUnknownVtbl){
    fn StrongNameDigestGenerate(
        wszFilePath: LPCWSTR, 
        ppbDigestBlob: *mut *mut BYTE, 
        pcbDigestBlob: *mut ULONG, 
        dwFlags: DWORD, 
    ) -> HRESULT,
    fn StrongNameDigestSign(
        wszKeyContainer: LPCWSTR, 
        pbKeyBlob: *mut BYTE, 
        cbKeyBlob: ULONG, 
        pbDigestBlob: *mut BYTE, 
        cbDigestBlob: ULONG, 
        hashAlgId: DWORD, 
        ppbSignatureBlob: *mut *mut BYTE, 
        pcbSignatureBlob: *mut ULONG, 
        dwFlags: DWORD,
    ) -> HRESULT,
    fn StrongNameDigestEmbed(
        wszFilePath: LPCWSTR, 
        pbSignatureBlob: *mut BYTE, 
        cbSignatureBlob: ULONG, 
    ) -> HRESULT,
}}
