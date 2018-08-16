#![allow(dead_code, non_upper_case_globals, non_camel_case_types, non_snake_case)]

use winapi::ctypes::{c_int, c_long, c_void};
use winapi::shared::basetsd::{SIZE_T, UINT64};
use winapi::shared::guiddef::{REFCLSID, REFIID};
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, HMODULE, LPVOID, UINT};
use winapi::shared::ntdef::{HANDLE, LPCSTR, LPCWSTR, LPWSTR, WCHAR, ULONGLONG};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::BSTR;

use winapi::um::objidlbase::IStream;
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::um::winnt::EXCEPTION_POINTERS;

use activation::IActivationFactory;
use gchost::COR_GC_STATS;

pub type HWND = *mut c_void;
DEFINE_GUID!(CLSID_CorRuntimeHost, 0xcb2f6723, 0xab3a, 0x11d2, 0x9c, 0x40, 0x00, 0xc0, 0x4f, 0xa3, 0x0a, 0x3e);
DEFINE_GUID!(CLSID_TypeNameFactory, 0xB81FF171, 0x20F3, 0x11d2, 0x8d, 0xcc, 0x00, 0xa0, 0xc9, 0xb0, 0x05, 0x25);
DEFINE_GUID!(CLSID_CLRRuntimeHost, 0x90F1A06E, 0x7712, 0x4762, 0x86, 0xB5, 0x7A, 0x5E, 0xBA, 0x6B, 0xDB, 0x02);
DEFINE_GUID!(CLSID_ComCallUnmarshal, 0x3F281000,0xE95A,0x11d2,0x88,0x6B,0x00,0xC0,0x4F,0x86,0x9F,0x04);
DEFINE_GUID!(CLSID_ComCallUnmarshalV4, 0x45fb4600,0xe6e8,0x4928,0xb2,0x5e,0x50,0x47,0x6f,0xf7,0x94,0x25);
DEFINE_GUID!(IID_IObjectHandle, 0xc460e2b4, 0xe199, 0x412a, 0x84, 0x56, 0x84, 0xdc, 0x3e, 0x48, 0x38, 0xc3);
DEFINE_GUID!(IID_IManagedObject, 0xc3fcc19e, 0xa970, 0x11d2, 0x8b, 0x5a, 0x00, 0xa0, 0xc9, 0xb7, 0xc9, 0xc4);
DEFINE_GUID!(IID_IApartmentCallback, 0x178e5337, 0x1528, 0x4591, 0xb1, 0xc9, 0x1c, 0x6e, 0x48, 0x46, 0x86, 0xd8);
DEFINE_GUID!(IID_ICatalogServices, 0x04c6be1e, 0x1db1, 0x4058, 0xab, 0x7a, 0x70, 0x0c, 0xcc, 0xfb, 0xf2, 0x54);
DEFINE_GUID!(IID_ICorRuntimeHost, 0xcb2f6722, 0xab3a, 0x11d2, 0x9c, 0x40, 0x00, 0xc0, 0x4f, 0xa3, 0x0a, 0x3e);
DEFINE_GUID!(IID_ICorThreadpool, 0x84680D3A, 0xB2C1, 0x46e8, 0xAC, 0xC2, 0xDB, 0xC0, 0xA3, 0x59, 0x15, 0x9A);
DEFINE_GUID!(IID_ICLRDebugManager, 0xdcaec6, 0x2ac0, 0x43a9, 0xac, 0xf9, 0x1e, 0x36, 0xc1, 0x39, 0xb1, 0xd);
DEFINE_GUID!(IID_IHostMemoryNeededCallback, 0x47EB8E57, 0x0846, 0x4546, 0xAF, 0x76, 0x6F, 0x42, 0xFC, 0xFC, 0x26, 0x49);
DEFINE_GUID!(IID_IHostMalloc, 0x1831991C, 0xCC53, 0x4A31, 0xB2, 0x18, 0x04, 0xE9, 0x10, 0x44, 0x64, 0x79);
DEFINE_GUID!(IID_IHostMemoryManager, 0x7BC698D1, 0xF9E3, 0x4460, 0x9C, 0xDE, 0xD0, 0x42, 0x48, 0xE9, 0xFA, 0x25);
DEFINE_GUID!(IID_ICLRTask, 0x28E66A4A, 0x9906, 0x4225, 0xB2, 0x31, 0x91, 0x87, 0xc3, 0xeb, 0x86, 0x11);
DEFINE_GUID!(IID_ICLRTask2, 0x28E66A4A, 0x9906, 0x4225, 0xB2, 0x31, 0x91, 0x87, 0xc3, 0xeb, 0x86, 0x12);
DEFINE_GUID!(IID_IHostTask, 0xC2275828, 0xC4B1, 0x4B55, 0x82, 0xC9, 0x92, 0x13, 0x5F, 0x74, 0xDF, 0x1A);
DEFINE_GUID!(IID_ICLRTaskManager, 0x4862efbe, 0x3ae5, 0x44f8, 0x8F, 0xEB, 0x34, 0x61, 0x90, 0xeE, 0x8A, 0x34);
DEFINE_GUID!(IID_IHostTaskManager, 0x997FF24C, 0x43B7, 0x4352, 0x86, 0x67, 0x0D, 0xC0, 0x4F, 0xAF, 0xD3, 0x54);
DEFINE_GUID!(IID_IHostThreadpoolManager, 0x983D50E2, 0xCB15, 0x466B, 0x80, 0xFC, 0x84, 0x5D, 0xC6, 0xE8, 0xC5, 0xFD);
DEFINE_GUID!(IID_ICLRIoCompletionManager, 0x2D74CE86, 0xB8D6, 0x4C84, 0xB3, 0xA7, 0x97, 0x68, 0x93, 0x3B, 0x3C, 0x12);
DEFINE_GUID!(IID_IHostIoCompletionManager, 0x8BDE9D80, 0xEC06, 0x41D6, 0x83, 0xE6, 0x22, 0x58, 0x0E, 0xFF, 0xCC, 0x20);
DEFINE_GUID!(IID_IHostSyncManager, 0x234330c7, 0x5f10, 0x4f20, 0x96, 0x15, 0x51, 0x22, 0xda, 0xb7, 0xa0, 0xac);
DEFINE_GUID!(IID_IHostCrst, 0x6DF710A6, 0x26A4, 0x4a65, 0x8c, 0xd5, 0x72, 0x37, 0xb8, 0xbd, 0xa8, 0xdc);
DEFINE_GUID!(IID_IHostAutoEvent, 0x50B0CFCE, 0x4063, 0x4278, 0x96, 0x73, 0xe5, 0xcb, 0x4e, 0xd0, 0xbd, 0xb8);
DEFINE_GUID!(IID_IHostManualEvent, 0x1BF4EC38, 0xAFFE, 0x4fb9, 0x85, 0xa6, 0x52, 0x52, 0x68, 0xf1, 0x5b, 0x54);
DEFINE_GUID!(IID_IHostSemaphore, 0x855efd47, 0xcc09, 0x463a, 0xa9, 0x7d, 0x16, 0xac, 0xab, 0x88, 0x26, 0x61);
DEFINE_GUID!(IID_ICLRSyncManager, 0x55FF199D, 0xAD21, 0x48f9, 0xa1, 0x6c, 0xf2, 0x4e, 0xbb, 0xb8, 0x72, 0x7d);
DEFINE_GUID!(IID_ICLRAppDomainResourceMonitor, 0xC62DE18C, 0x2E23, 0x4AEA, 0x84, 0x23, 0xB4, 0x0C, 0x1F, 0xC5, 0x9E, 0xAE);
DEFINE_GUID!(IID_ICLRPolicyManager, 0x7D290010, 0xD781, 0x45da, 0xA6, 0xF8, 0xAA, 0x5D, 0x71, 0x1A, 0x73, 0x0E);
DEFINE_GUID!(IID_ICLRGCManager, 0x54D9007E, 0xA8E2, 0x4885, 0xB7, 0xBF, 0xF9, 0x98, 0xDE, 0xEE, 0x4F, 0x2A);
DEFINE_GUID!(IID_ICLRGCManager2, 0x0603B793, 0xA97A, 0x4712, 0x9C, 0xB4, 0x0C, 0xD1, 0xC7, 0x4C, 0x0F, 0x7C);
DEFINE_GUID!(IID_ICLRErrorReportingManager, 0x980d2f1a, 0xbf79, 0x4c08, 0x81, 0x2a, 0xbb, 0x97, 0x78, 0x92, 0x8f, 0x78);
DEFINE_GUID!(IID_ICLRErrorReportingManager2, 0xc68f63b1, 0x4d8b, 0x4e0b, 0x95, 0x64, 0x9d, 0x2e, 0xfe, 0x2f, 0xa1, 0x8c);
DEFINE_GUID!(IID_IHostPolicyManager, 0x7AE49844, 0xB1E3, 0x4683, 0xBA, 0x7C, 0x1E, 0x82, 0x12, 0xEA, 0x3B, 0x79);
DEFINE_GUID!(IID_IHostGCManager, 0x5D4EC34E, 0xF248, 0x457B, 0xB6, 0x03, 0x25, 0x5F, 0xAA, 0xBA, 0x0D, 0x21);
DEFINE_GUID!(IID_IActionOnCLREvent, 0x607BE24B, 0xD91B, 0x4E28, 0xA2, 0x42, 0x61, 0x87, 0x1C, 0xE5, 0x6E, 0x35);
DEFINE_GUID!(IID_ICLROnEventManager, 0x1D0E0132, 0xE64F, 0x493D, 0x92, 0x60, 0x02, 0x5C, 0x0E, 0x32, 0xC1, 0x75);
DEFINE_GUID!(IID_ICLRHostProtectionManager, 0x89f25f5c, 0xceef, 0x43e1, 0x9c, 0xfa, 0xa6, 0x8c, 0xe8, 0x63, 0xaa, 0xac);
DEFINE_GUID!(IID_IHostAssemblyStore, 0x7b102a88, 0x3f7f, 0x496d, 0x8f, 0xa2, 0xc3, 0x53, 0x74, 0xe0, 0x1a, 0xf3);
DEFINE_GUID!(IID_IHostAssemblyManager, 0x613dabd7, 0x62b2, 0x493e, 0x9e, 0x65, 0xc1, 0xe3, 0x2a, 0x1e, 0x0c, 0x5e);
DEFINE_GUID!(IID_IHostSecurityManager, 0x75ad2468, 0xa349, 0x4d02, 0xa7, 0x64, 0x76, 0xa6, 0x8a, 0xee, 0x0c, 0x4f);
DEFINE_GUID!(IID_IHostSecurityContext, 0x7e573ce4, 0x343, 0x4423, 0x98, 0xd7, 0x63, 0x18, 0x34, 0x8a, 0x1d, 0x3c);
DEFINE_GUID!(IID_ICLRAssemblyIdentityManager, 0x15f0a9da, 0x3ff6, 0x4393, 0x9d, 0xa9, 0xfd, 0xfd, 0x28, 0x4e, 0x69, 0x72);
DEFINE_GUID!(IID_ICLRDomainManager, 0x270d00a2, 0x8e15, 0x4d0b, 0xad, 0xeb, 0x37, 0xbc, 0x3e, 0x47, 0xdf, 0x77);
DEFINE_GUID!(IID_ICLRAssemblyReferenceList, 0x1b2c9750, 0x2e66, 0x4bda, 0x8b, 0x44, 0x0a, 0x64, 0x2c, 0x5c, 0xd7, 0x33);
DEFINE_GUID!(IID_ICLRReferenceAssemblyEnum, 0xd509cb5d, 0xcf32, 0x4876, 0xae, 0x61, 0x67, 0x77, 0x0c, 0xf9, 0x19, 0x73);
DEFINE_GUID!(IID_ICLRProbingAssemblyEnum, 0xd0c5fb1f, 0x416b, 0x4f97, 0x81, 0xf4, 0x7a, 0xc7, 0xdc, 0x24, 0xdd, 0x5d);
DEFINE_GUID!(IID_ICLRHostBindingPolicyManager, 0x4b3545e7, 0x1856, 0x48c9, 0xa8, 0xba, 0x24, 0xb2, 0x1a, 0x75, 0x3c, 0x09);
DEFINE_GUID!(IID_ICLRRuntimeHost, 0x90F1A06C, 0x7712, 0x4762, 0x86, 0xB5, 0x7A, 0x5E, 0xBA, 0x6B, 0xDB, 0x02);
DEFINE_GUID!(IID_ICLRRuntimeHost2, 0x712AB73F, 0x2C22, 0x4807, 0xAD, 0x7E, 0xF5, 0x01, 0xD7, 0xb7, 0x2C, 0x2D);
DEFINE_GUID!(IID_ICLRRuntimeHost4, 0x64F6D366, 0xD7C2, 0x4F1F, 0xB4, 0xB2, 0xE8, 0x16, 0x0C, 0xAC, 0x43, 0xAF);
DEFINE_GUID!(IID_ICLRExecutionManager, 0x1000A3E7, 0xB420, 0x4620, 0xAE, 0x30, 0xFB, 0x19, 0xB5, 0x87, 0xAD, 0x1D);
DEFINE_GUID!(IID_ITypeName, 0xB81FF171, 0x20F3, 0x11d2, 0x8d, 0xcc, 0x00, 0xa0, 0xc9, 0xb0, 0x05, 0x22);
DEFINE_GUID!(IID_ITypeNameBuilder, 0xB81FF171, 0x20F3, 0x11d2, 0x8d, 0xcc, 0x00, 0xa0, 0xc9, 0xb0, 0x05, 0x23);
DEFINE_GUID!(IID_ITypeNameFactory, 0xB81FF171, 0x20F3, 0x11d2, 0x8d, 0xcc, 0x00, 0xa0, 0xc9, 0xb0, 0x05, 0x21);

STDAPI!{
    fn GetCORSystemDirectory(
        phBuffer: LPWSTR, cchBuffer: DWORD, dwLength: *mut DWORD,
    ) -> HRESULT
}
STDAPI! {
    fn GetCORVersion(
        szBuffer: LPWSTR, cchBuffer: DWORD, dwLength: *mut DWORD,
    ) -> HRESULT
}
STDAPI!{
    fn GetFileVersion(
        szFilename: LPCWSTR, szBuffer: LPWSTR, cchBuffer: DWORD, dwLength: *mut DWORD,
    ) -> HRESULT
}
STDAPI! {
    fn GetCORRequiredVersion(
        pbuffer: LPWSTR, cchBuffer: DWORD, dwLength: *mut DWORD,
    ) -> HRESULT
}
STDAPI! {
    fn GetRequestedRuntimeInfo(
        pExe: LPCWSTR, 
        pwszVersion: LPCWSTR, 
        pConfigurationFile: LPCWSTR, 
        startupFlags: DWORD, 
        runtimeInfoFlags: DWORD, 
        pDirectory: LPWSTR, 
        dwDirectory: DWORD,
        dwDirectoryLength: *mut  DWORD,
        pVersion: LPWSTR, 
        cchBuffer: DWORD, 
        dwlength: *mut DWORD,
    ) -> HRESULT
}
STDAPI!{fn GetRequestedRuntimeVersion(
    pExe: LPWSTR,
    pVersion: LPWSTR, 
    cchBuffer: DWORD,
    dwLength: *mut DWORD,
) -> HRESULT}
STDAPI!{fn CorBindToRuntimeHost(
    pwszVersion: LPCWSTR, 
    pwszBuildFlavor: LPCWSTR, 
    pwszHostConfigFile: LPCWSTR,
    pReserved: *mut c_void, 
    startupFlags: DWORD, 
    rclsid: REFCLSID, 
    riid: REFIID, 
    ppv: *mut LPVOID,
) -> HRESULT}
STDAPI!{fn CorBindToRuntimeEx(
    pwszVersion: LPCWSTR, 
    pwszBuildFlavor: LPCWSTR, 
    startupFlags: DWORD, 
    rclsid: REFCLSID, 
    riid: REFIID, 
    ppv: *mut LPVOID,
) -> HRESULT}
STDAPI!{fn CorBindToRuntimeByCfg(
    pCfgStream: *mut IStream, 
    reserved: DWORD, 
    startupFlags: DWORD, 
    rclsid: REFCLSID,
    riid: REFIID, 
    ppv: *mut LPVOID,
) -> HRESULT}
STDAPI!{fn CorBindToRuntime(
    pwszVersion: LPCWSTR, 
    pwszBuildFlavor: LPCWSTR, 
    rclsid: REFCLSID, 
    riid: REFIID, 
    ppv: *mut LPVOID,
) -> HRESULT}
STDAPI!{fn CorBindToCurrentRuntime(
    pwszFileName: LPCWSTR, 
    rclsid: REFCLSID, 
    riid: REFIID, 
    ppv: *mut LPVOID,
) -> HRESULT}
STDAPI!{fn ClrCreateManagedInstance(
    pTypeName: LPCWSTR, 
    riid: REFIID, 
    ppObject: *mut *mut c_void,
) -> HRESULT}
STDAPI!{fn CorMarkThreadInThreadPool() -> HRESULT}
STDAPI!{fn RunDll32ShimW(
    hwnd: HWND, 
    hinst: HINSTANCE, 
    lpszCmdLine: LPCWSTR, 
    nCmdShow: i32,
) -> HRESULT}
STDAPI!{fn LoadLibraryShim(
    szDllName: LPCWSTR, 
    szVersion: LPCWSTR, 
    pvReserved: LPVOID, 
    phModDll: *mut HMODULE,
) -> HRESULT}
STDAPI!{fn CallFunctionShim(
    szDllName: LPCWSTR, 
    szFunctionName: LPCSTR, 
    lpvArgument1: LPVOID, 
    lpvArgument2: LPVOID, 
    szVersion: LPCWSTR, 
    pvReserved: LPVOID,
) -> HRESULT}
STDAPI!{fn GetRealProcAddress(
    pwszProcName: LPCSTR, 
    ppv: *mut *mut c_void,
) -> HRESULT}
STDAPI!{fn CorExitProcess(
    exitCode: i32,
) -> HRESULT}
STDAPI!{fn LoadStringRC(
    iResouceID: UINT, 
    szBuffer: LPWSTR, 
    iMax: i32, 
    bQuiet: i32,
) -> HRESULT}

pub type FnGetCLRRuntimeHost = fn(REFIID, *mut *mut IUnknown) -> HRESULT;

ENUM!{enum __MIDL___MIDL_itf_mscoree_0000_0000_0001
{
    HOST_TYPE_DEFAULT	= 0,
    HOST_TYPE_APPLAUNCH	= 0x1,
    HOST_TYPE_CORFLAG	= 0x2,
}}

pub type HOST_TYPE = __MIDL___MIDL_itf_mscoree_0000_0000_0001;

#[repr(C)]
pub struct _PROCESS_INFORMATION{
    hProcess: HANDLE, 
    hThread: HANDLE, 
    dwProcessId: DWORD, 
    dwThreadId: DWORD, 
} 
pub type LPPROCESS_INFORMATION = *mut _PROCESS_INFORMATION; 
STDAPI!{fn CorLaunchApplication(
    dwClickOnceHost: HOST_TYPE,
    pwzAppFullName: LPCWSTR,
    dwManifestPaths: DWORD, 
    ppwzManifestPaths: *mut LPCWSTR,
    dwActivationData: DWORD, 
    ppwzActivationData: *mut LPCWSTR,
    lpProcessInformation: LPPROCESS_INFORMATION,
) -> HRESULT}

pub type FExecuteInAppDomainCallback = fn(*mut c_void) -> HRESULT;

ENUM!{enum __MIDL___MIDL_itf_mscoree_0000_0000_0002
{
    STARTUP_CONCURRENT_GC = 0x1,
    STARTUP_LOADER_OPTIMIZATION_MASK = 0x3 << 1,
    STARTUP_LOADER_OPTIMIZATION_SINGLE_DOMAIN = 0x1 << 1,
    STARTUP_LOADER_OPTIMIZATION_MULTI_DOMAIN = 0x2 << 1,
    STARTUP_LOADER_OPTIMIZATION_MULTI_DOMAIN_HOST =  0x3 << 1,
    STARTUP_LOADER_SAFEMODE	= 0x10,
    STARTUP_LOADER_SETPREFERENCE	= 0x100,
    STARTUP_SERVER_GC	= 0x1000,
    STARTUP_HOARD_GC_VM	= 0x2000,
    STARTUP_SINGLE_VERSION_HOSTING_INTERFACE	= 0x4000,
    STARTUP_LEGACY_IMPERSONATION	= 0x10000,
    STARTUP_DISABLE_COMMITTHREADSTACK	= 0x20000,
    STARTUP_ALWAYSFLOW_IMPERSONATION	= 0x40000,
    STARTUP_TRIM_GC_COMMIT	= 0x80000,
    STARTUP_ETW	= 0x100000,
    STARTUP_ARM	= 0x400000,
    STARTUP_SINGLE_APPDOMAIN	= 0x800000,
    STARTUP_APPX_APP_MODEL	= 0x1000000,
    STARTUP_DISABLE_RANDOMIZED_STRING_HASHING	= 0x2000000, // not supported
}}
pub type STARTUP_FLAGS = __MIDL___MIDL_itf_mscoree_0000_0000_0002;

ENUM!{enum __MIDL___MIDL_itf_mscoree_0000_0000_0003
{
    CLSID_RESOLUTION_DEFAULT	= 0,
    CLSID_RESOLUTION_REGISTERED	= 0x1,
}}
pub type CLSID_RESOLUTION_FLAGS = __MIDL___MIDL_itf_mscoree_0000_0000_0003;

ENUM!{enum __MIDL___MIDL_itf_mscoree_0000_0000_0004
{
    RUNTIME_INFO_UPGRADE_VERSION	= 0x1,
    RUNTIME_INFO_REQUEST_IA64	= 0x2,
    RUNTIME_INFO_REQUEST_AMD64	= 0x4,
    RUNTIME_INFO_REQUEST_X86	= 0x8,
    RUNTIME_INFO_DONT_RETURN_DIRECTORY	= 0x10,
    RUNTIME_INFO_DONT_RETURN_VERSION	= 0x20,
    RUNTIME_INFO_DONT_SHOW_ERROR_DIALOG	= 0x40,
    RUNTIME_INFO_IGNORE_ERROR_MODE	= 0x1000,
}}
pub type RUNTIME_INFO_FLAGS = __MIDL___MIDL_itf_mscoree_0000_0000_0004;

ENUM!{enum __MIDL___MIDL_itf_mscoree_0000_0000_0005
{
    APPDOMAIN_SECURITY_DEFAULT	= 0,
    APPDOMAIN_SECURITY_SANDBOXED	= 0x1,
    APPDOMAIN_SECURITY_FORBID_CROSSAD_REVERSE_PINVOKE	= 0x2,
    APPDOMAIN_IGNORE_UNHANDLED_EXCEPTIONS	= 0x4,
    APPDOMAIN_FORCE_TRIVIAL_WAIT_OPERATIONS	= 0x8,
    APPDOMAIN_ENABLE_PINVOKE_AND_CLASSIC_COMINTEROP	= 0x10,
    APPDOMAIN_SET_TEST_KEY	= 0x20,
    APPDOMAIN_ENABLE_PLATFORM_SPECIFIC_APPS	= 0x40,
    APPDOMAIN_ENABLE_ASSEMBLY_LOADFILE	= 0x80,
    APPDOMAIN_DISABLE_TRANSPARENCY_ENFORCEMENT	= 0x100,
}}
pub type APPDOMAIN_SECURITY_FLAGS = __MIDL___MIDL_itf_mscoree_0000_0000_0005;

STDAPI!{fn GetRequestedRuntimeVersionForCLSID(
    rclsid: REFCLSID, 
    pVersion: LPWSTR, 
    cchBuffer: DWORD, 
    dwLength: *mut DWORD, 
    dwResolutionFlags: CLSID_RESOLUTION_FLAGS,
) -> HRESULT}

DEFINE_GUID!(IID_IDebuggerThreadControl, 0x23d86786, 0x0bb5, 0x4774, 0x8f, 0xb5, 0xe3, 0x52, 0x2a, 0xdd, 0x62, 0x46);

RIDL!{#[uuid(0x23D86786, 0x0BB5, 0x4774, 0x8F, 0xB5, 0xE3, 0x52, 0x2A, 0xDD, 0x62, 0x46)]
interface IDebuggerThreadControl(IDebuggerThreadControlVtbl): IUnknown(IUnknownVtbl){
    fn ThreadIsBlockingForDebugger() -> HRESULT,
    fn ReleaseAllRuntimeThreads() -> HRESULT, 
    fn StartBlockingForDebugger(
        dwUnused: DWORD,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xBF24142D, 0xA47D, 0x4d24, 0xA6, 0x6D, 0x8C, 0x21, 0x41, 0x94, 0x4E, 0x44)]
interface IDebuggerInfo(IDebuggerInfoVtbl): IUnknown(IUnknownVtbl){
    fn IsDebuggerAttached(
        pbAttached: *mut BOOL, 
    ) -> HRESULT,
}}

pub type HDOMNAINENUM = *mut c_void;

ENUM!{enum __MIDL___MIDL_itf_mscoree_0000_0002_0001
{
    eMemoryAvailableLow	= 1,
    eMemoryAvailableNeutral	= 2,
    eMemoryAvailableHigh	= 3,
}}
pub type EMemoryAvailable = __MIDL___MIDL_itf_mscoree_0000_0002_0001;

ENUM!{enum __MIDL___MIDL_itf_mscoree_0000_0002_0002
{
    eTaskCritical	= 0,
    eAppDomainCritical	= 1,
    eProcessCritical	= 2,
}}
pub type EMemoryCriticalLevel = __MIDL___MIDL_itf_mscoree_0000_0002_0002;

ENUM!{enum __MIDL___MIDL_itf_mscoree_0000_0002_0003
{
    WAIT_MSGPUMP	= 0x1,
    WAIT_ALERTABLE	= 0x2,
    WAIT_NOTINDEADLOCK	= 0x4,
}}
pub type WAIT_OPTION = __MIDL___MIDL_itf_mscoree_0000_0002_0003;
pub type TASKID = UINT64;
pub type CONNID = DWORD;

ENUM!{enum ETaskType
{
    TT_DEBUGGERHELPER	= 0x1,
    TT_GC	= 0x2,
    TT_FINALIZER	= 0x4,
    TT_THREADPOOL_TIMER	= 0x8,
    TT_THREADPOOL_GATE	= 0x10,
    TT_THREADPOOL_WORKER	= 0x20,
    TT_THREADPOOL_IOCOMPLETION	= 0x40,
    TT_ADUNLOAD	= 0x80,
    TT_USER	= 0x100,
    TT_THREADPOOL_WAIT	= 0x200,
    TT_UNKNOWN	= 0x80000000,
}}

ENUM!{enum __MIDL___MIDL_itf_mscoree_0000_0002_0004
{
    eSymbolReadingNever	= 0,
    eSymbolReadingAlways	= 1,
    eSymbolReadingFullTrustOnly	= 2,
}}
pub type ESymbolReadingPolicy = __MIDL___MIDL_itf_mscoree_0000_0002_0004;

ENUM!{enum __MIDL___MIDL_itf_mscoree_0000_0002_0005
{
    DUMP_FLAVOR_Mini	= 0,
    DUMP_FLAVOR_CriticalCLRState	= 1,
    DUMP_FLAVOR_NonHeapCLRState	= 2,
    DUMP_FLAVOR_Default	= DUMP_FLAVOR_Mini,
}}
pub type ECustomDumpFlavor = __MIDL___MIDL_itf_mscoree_0000_0002_0005;

ENUM!{enum __MIDL___MIDL_itf_mscoree_0000_0002_0006
{
    DUMP_ITEM_None	= 0,
}}
pub type ECustomDumpItemKind = __MIDL___MIDL_itf_mscoree_0000_0002_0006;

STRUCT!{struct __MIDL___MIDL_itf_mscoree_0000_0002_0007{
    itemKind: ECustomDumpItemKind,
    pReserved: UINT,
}}
pub type CustomDumpItem = __MIDL___MIDL_itf_mscoree_0000_0002_0007;

ENUM!{enum __MIDL___MIDL_itf_mscoree_0000_0002_0009
{
    Parameter1	= 0,
    Parameter2	= ( Parameter1 + 1 ) ,
    Parameter3	= ( Parameter2 + 1 ) ,
    Parameter4	= ( Parameter3 + 1 ) ,
    Parameter5	= ( Parameter4 + 1 ) ,
    Parameter6	= ( Parameter5 + 1 ) ,
    Parameter7	= ( Parameter6 + 1 ) ,
    Parameter8	= ( Parameter7 + 1 ) ,
    Parameter9	= ( Parameter8 + 1 ) ,
    InvalidBucketParamIndex	= ( Parameter9 + 1 ) ,
}}
pub type BucketParameterIndex = __MIDL___MIDL_itf_mscoree_0000_0002_0009;

STRUCT!{struct _BucketParameters
{
    fInited: BOOL,
    pszEventTypeName: [WCHAR; 255],
    pszParams: [[WCHAR;255]; 10],
}}
pub type BucketParameters = _BucketParameters;

RIDL!{#[uuid(0x980D2F1A, 0xBF79, 0x4c08, 0x81, 0x2A, 0xBB, 0x97, 0x78, 0x92, 0x8F, 0x78)]
interface ICLRErrorReportingManager(ICLRErrorReportingManagerVtbl): IUnknown(IUnknownVtbl){
    fn GetBucketParametersForCurrentException(
        pParams: *mut BucketParameters,
    ) -> HRESULT,
    fn BeginCustomDump(
        dwFlave: ECustomDumpFlavor,
        dwNumItems: DWORD, 
        items: *mut CustomDumpItem, 
        dwReserved: DWORD,
    ) -> HRESULT, 
    fn EndCustomDump() -> HRESULT,
}}

ENUM!{enum __MIDL___MIDL_itf_mscoree_0000_0003_0001
{
    ApplicationID	= 0x1,
    InstanceID	= 0x2,
}}
pub type ApplicationDataKey = __MIDL___MIDL_itf_mscoree_0000_0003_0001;

RIDL!{#[uuid(0xC68F63B1, 0x4D8B, 0x4E0B, 0x95, 0x64, 0x9D, 0x2E, 0xFE, 0x2F, 0xA1, 0x8C)]
interface ICLRErrorReportingManager2(ICLRErrorReportingManager2Vtbl): ICLRErrorReportingManager(ICLRErrorReportingManagerVtbl){
    fn SetApplicationData(
        key: ApplicationDataKey, 
        pValue: *const WCHAR,
    ) -> HRESULT,
    fn SetBucketParametersForUnhandledException(
        pBucketParams: *const BucketParameters, 
        pCountParams: *mut DWORD,
    ) -> HRESULT,
}}

ENUM!{enum __MIDL___MIDL_itf_mscoree_0000_0004_0001
{
    OPR_ThreadAbort	= 0,
    OPR_ThreadRudeAbortInNonCriticalRegion	= ( OPR_ThreadAbort + 1 ) ,
    OPR_ThreadRudeAbortInCriticalRegion	= ( OPR_ThreadRudeAbortInNonCriticalRegion + 1 ) ,
    OPR_AppDomainUnload	= ( OPR_ThreadRudeAbortInCriticalRegion + 1 ) ,
    OPR_AppDomainRudeUnload	= ( OPR_AppDomainUnload + 1 ) ,
    OPR_ProcessExit	= ( OPR_AppDomainRudeUnload + 1 ) ,
    OPR_FinalizerRun	= ( OPR_ProcessExit + 1 ) ,
    MaxClrOperation	= ( OPR_FinalizerRun + 1 ),
}}
pub type EClrOperation = __MIDL___MIDL_itf_mscoree_0000_0004_0001;

ENUM!{enum __MIDL___MIDL_itf_mscoree_0000_0004_0002
{
    FAIL_NonCriticalResource	= 0,
    FAIL_CriticalResource	= ( FAIL_NonCriticalResource + 1 ) ,
    FAIL_FatalRuntime	= ( FAIL_CriticalResource + 1 ) ,
    FAIL_OrphanedLock	= ( FAIL_FatalRuntime + 1 ) ,
    FAIL_StackOverflow	= ( FAIL_OrphanedLock + 1 ) ,
    FAIL_AccessViolation	= ( FAIL_StackOverflow + 1 ) ,
    FAIL_CodeContract	= ( FAIL_AccessViolation + 1 ) ,
    MaxClrFailure	= ( FAIL_CodeContract + 1 ), 
}}
pub type EClrFailure = __MIDL___MIDL_itf_mscoree_0000_0004_0002;

ENUM!{enum __MIDL___MIDL_itf_mscoree_0000_0004_0003
{
    eRuntimeDeterminedPolicy	= 0,
    eHostDeterminedPolicy	= ( eRuntimeDeterminedPolicy + 1 ), 
}}
pub type EClrUnhandledException = __MIDL___MIDL_itf_mscoree_0000_0004_0003;

ENUM!{enum __MIDL___MIDL_itf_mscoree_0000_0004_0004
{
    eNoAction	= 0,
    eThrowException	= ( eNoAction + 1 ) ,
    eAbortThread	= ( eThrowException + 1 ) ,
    eRudeAbortThread	= ( eAbortThread + 1 ) ,
    eUnloadAppDomain	= ( eRudeAbortThread + 1 ) ,
    eRudeUnloadAppDomain	= ( eUnloadAppDomain + 1 ) ,
    eExitProcess	= ( eRudeUnloadAppDomain + 1 ) ,
    eFastExitProcess	= ( eExitProcess + 1 ) ,
    eRudeExitProcess	= ( eFastExitProcess + 1 ) ,
    eDisableRuntime	= ( eRudeExitProcess + 1 ) ,
    MaxPolicyAction	= ( eDisableRuntime + 1 ), 
}}
pub type EPolicyAction = __MIDL___MIDL_itf_mscoree_0000_0004_0004;

RIDL!{#[uuid(0x7D290010, 0xD781, 0x45da, 0xA6, 0xF8, 0xAA, 0x5D, 0x71, 0x1A, 0x73, 0x0E)]
interface ICLRPolicyManager(ICLRPolicyManagerVtbl): IUnknown(IUnknownVtbl){
    fn SetDefaultAction(
        operation: EClrOperation, 
        action: EPolicyAction, 
    ) -> HRESULT,
    fn SetTimeout(
        operation: EClrOperation,
        dwMilliseconds: DWORD, 
    ) -> HRESULT,
    fn SetActionOnTimeout(
        operation: EClrOperation, 
        action: EPolicyAction, 
    ) -> HRESULT, 
    fn SetTimeoutAndAction(
        operation: EClrOperation, 
        dwMilliseconds: DWORD,
        action: EPolicyAction, 
    ) -> HRESULT, 
    fn SetActionOnFailure(
        failure: EClrFailure,
        action: EPolicyAction,  
    ) -> HRESULT,
    fn SetUnhandledExceptionPolicy(
        policy: EClrUnhandledException,
    ) -> HRESULT,
}}

ENUM!{enum __MIDL___MIDL_itf_mscoree_0000_0005_0001
{
    Event_DomainUnload	= 0,
    Event_ClrDisabled	= ( Event_DomainUnload + 1 ) ,
    Event_MDAFired	= ( Event_ClrDisabled + 1 ) ,
    Event_StackOverflow	= ( Event_MDAFired + 1 ) ,
    MaxClrEvent	= ( Event_StackOverflow + 1 ) ,
}}
pub type EClrEvent = __MIDL___MIDL_itf_mscoree_0000_0005_0001;

STRUCT!{struct _MDAInfo
{
    lpMDACaption: LPCWSTR,
    lpMDAMessage: LPCWSTR,
    lpStackTrace: LPCWSTR,
}}
pub type MDAInfo = _MDAInfo;

ENUM!{enum __MIDL___MIDL_itf_mscoree_0000_0005_0002
{
    SO_Managed	= 0,
    SO_ClrEngine	= ( SO_Managed + 1 ) ,
    SO_Other	= ( SO_ClrEngine + 1 ) ,
}}
pub type StackOverflowType = __MIDL___MIDL_itf_mscoree_0000_0005_0002;

STRUCT!{struct _StackOverflowInfo
{
    soType: StackOverflowType,
    pExceptionInfo: *mut EXCEPTION_POINTERS,
}}
pub type StackOverflowInfo = _StackOverflowInfo;

RIDL!{#[uuid(0x54D9007E, 0xA8E2, 0x4885, 0xB7, 0xBF, 0xF9, 0x98, 0xDE, 0xEE, 0x4F, 0x2A)]
interface ICLRGCManager(ICLRGCManagerVtbl): IUnknown(IUnknownVtbl){
    fn Collect(
        Generation: c_long, 
    ) -> HRESULT,
    fn GetStats(
        pStats: *mut COR_GC_STATS,
    ) -> HRESULT,
    fn SetGCStartupLimits(
        SegmentSize: DWORD, 
        MaxGen0Size: DWORD,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x0603B793, 0xA97A, 0x4712, 0x9C, 0xB4, 0x0C, 0xD1, 0xC7, 0x4C, 0x0F, 0x7C)]
interface ICLRGCManager2(ICLRGCManager2Vtbl): ICLRGCManager(ICLRGCManagerVtbl){
    fn SetGCStartupLimitEx(
        SegmentSize: SIZE_T, 
        MaxGen0Size: SIZE_T,
    ) -> HRESULT,
}}

ENUM!{enum __MIDL___MIDL_itf_mscoree_0000_0007_0001
{
    ePolicyLevelNone	= 0,
    ePolicyLevelRetargetable	= 0x1,
    ePolicyUnifiedToCLR	= 0x2,
    ePolicyLevelApp	= 0x4,
    ePolicyLevelPublisher	= 0x8,
    ePolicyLevelHost	= 0x10,
    ePolicyLevelAdmin	= 0x20,
    ePolicyPortability	= 0x40, 
}}
pub type EBindPolicyLevels = __MIDL___MIDL_itf_mscoree_0000_0007_0001;

STRUCT!{struct _AssemblyBindInfo
{
    dwAppDomainId: DWORD,
    lpReferencedIdentity: LPCWSTR,
    lpPostPolicyIdentity: LPCWSTR,
    ePolicyLevel: DWORD,
}}
pub type AssemblyBindInfo = _AssemblyBindInfo;

STRUCT!{struct _ModuleBindInfo
{
    dwAppDomainId: DWORD,
    lpAssemblyIdentity: LPCWSTR,
    lpModuleName: LPCWSTR,
}}
pub type ModuleBindInfo = _ModuleBindInfo;

ENUM!{enum _HostApplicationPolicy
{
    HOST_APPLICATION_BINDING_POLICY	= 1,
}}
pub type EHostApplicationPolicy = _HostApplicationPolicy;

STDAPI!{fn GetCLRIdentityManager(
    riid: REFIID, 
    ppManager: *mut *mut IUnknown,
) -> HRESULT}

DEFINE_GUID!{IID_IHostControl, 0x02CA073C, 0x7079, 0x4860, 0x88, 0x0A, 0xC2, 0xF7, 0xA4, 0x49, 0xC9, 0x91}

RIDL!{#[uuid(0x02CA073C, 0x7079, 0x4860, 0x88, 0x0A, 0xC2, 0xF7, 0xA4, 0x49, 0xC9, 0x91)]
interface IHostControl(IHostControlVtbl): IUnknown(IUnknownVtbl){
    fn GetHostManager(
        riid: REFIID, 
        ppObject: *mut *mut c_void,
    ) -> HRESULT,
    fn SetAppDomainManager(
        dwAppDomainId: DWORD, 
        pUnkAppDomainManager: *mut IUnknown,
    ) -> HRESULT,
}}

DEFINE_GUID!{IID_ICLRControl, 0x9065597E, 0xD1A1, 0x4fb2, 0xB6, 0xBA, 0x7E, 0x1F, 0xCE, 0x23, 0x0F, 0x61}

RIDL!{#[uuid(0x9065597E, 0xD1A1, 0x4fb2, 0xB6, 0xBA, 0x7E, 0x1F, 0xCE, 0x23, 0x0F, 0x61)]
interface ICLRControl(ICLRControlVtbl): IUnknown(IUnknownVtbl){
    fn GetCLRManager(
        riid: REFIID, 
        ppObject: *mut *mut c_void,
    ) -> HRESULT, 
    fn SetAppDomainManagerType(
        pwzAppDomainManagerAssembly: LPCWSTR, 
        pwzAppDomainManagerType: LPCWSTR,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x90F1A06C, 0x7712, 0x4762, 0x86, 0xB5, 0x7A, 0x5E, 0xBA, 0x6B, 0xDB, 0x02)]
interface ICLRRuntimeHost(ICLRRuntimeHostVtbl): IUnknown(IUnknownVtbl){
    fn Start() -> HRESULT,
    fn Stop() -> HRESULT,
    fn SetHostControl(
        pHostControl: *mut IHostControl,
    ) -> HRESULT,
    fn UnloadAppDomain(
        dwAppDomainId: DWORD, 
        fWaitUntilDone: BOOL, 
    ) -> HRESULT,
    fn ExecuteInAppDomain(
        dwAppDomainId: DWORD, 
        pCallBack: FExecuteInAppDomainCallback, 
        cookie: *mut c_void,
    ) -> HRESULT,
    fn GetCurrentAppDomainId(
        pdwAppDomainId: *mut DWORD, 
    ) -> HRESULT, 
    fn ExecuteApplication(
        pwzAppFullName: LPCWSTR, 
        dwManifestPaths: DWORD, 
        ppwzManifestPaths: *mut LPCWSTR, 
        dwActivationData: DWORD, 
        ppwzActivationData: *mut LPCWSTR,
        pReturnValue: *mut c_int,
    ) -> HRESULT,
    fn ExecuteInDefaultAppDomain(
        pwzAssemblyPath: LPCWSTR, 
        pwzTypeName: LPCWSTR, 
        pwzMethodName: LPCWSTR, 
        pwzArgument: LPCWSTR, 
        pReturnValue: *mut DWORD, 
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x712AB73F, 0x2C22, 0x4807, 0xAD, 0x7E, 0xF5, 0x01, 0xD7, 0xB7, 0x2C, 0x2D)]
interface ICLRRuntimeHost2(ICLRRuntimeHost2Vtbl): ICLRRuntimeHost(ICLRRuntimeHostVtbl){
    fn CreateAppDomainWithManager(
        wszFriendlyName: LPCWSTR, 
        dwFlags: DWORD, 
        wszAppDomainManagerAssemblyName: LPCWSTR, 
        wszAppDomainManagerTypeName: LPCWSTR, 
        nProperties: i32, 
        pPropertyNames: *mut LPCWSTR, 
        pPropertyValues: *mut LPCWSTR, 
        pAppDomainId: *mut DWORD, 
    ) -> HRESULT, 
    fn CreateDelegate(
        appDomainID: DWORD, 
        wszAssemblyName: LPCWSTR, 
        wszClassName: LPCWSTR, 
        wszMethodName: LPCWSTR, 
        fnPtr: *mut i32,
    ) -> HRESULT,
    fn Authenticate(
        authKey: ULONGLONG,
    ) -> HRESULT, 
    fn RegisterMacEHPort() -> HRESULT, 
    fn SetStartupFlags(
        dwFlags: STARTUP_FLAGS,
    ) -> HRESULT, 
    fn DllGetActivationFactory(
        appDomainID: DWORD, 
        wszTypeName: LPCWSTR, 
        factory: *mut *mut IActivationFactory,
    ) -> HRESULT,
    fn ExecuteAssembly(
        dwAppDomainId: DWORD, 
        pwzAssemblyPath: LPCWSTR, 
        argc: i32, 
        argv: *mut LPCWSTR, 
        pReturnValue: *mut DWORD, 
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x64F6D366, 0xD7C2, 0x4F1F, 0xB4, 0xB2, 0xE8, 0x16, 0x0C, 0xAC, 0x43, 0xAF)]
interface ICLRRuntimeHost4(ICLRRuntimeHost4Vtbl): ICLRRuntimeHost2(ICLRRuntimeHost2Vtbl){
    fn UnloadAppDomain2(
        dwAppDomainId: DWORD, 
        fWaitUntilDone: BOOL, 
        pLatchedExitCode: *mut i32, 
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x1000A3E7, 0xB420, 0x4620, 0xAE, 0x30, 0xFB, 0x19, 0xB5, 0x87, 0xAD, 0x1D)]
interface ICLRExecutionManager(ICLRExecutionManagerVtbl): IUnknown(IUnknownVtbl){
    fn Pause(
        dwAppDomainId: DWORD, 
        dwFlags: DWORD, 
    ) -> HRESULT,
    fn Resume(
        dwAppDomainId: DWORD, 
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xF2833A0C, 0xF944, 0x48d8, 0x94, 0x0E, 0xF5, 0x94, 0x25, 0xED, 0xBF, 0xCF)]
interface IHostNetCFDebugControlManager(IHostNetCFDebugControlManagerVtbl): IUnknown(IUnknownVtbl){
    fn NotifyPause(
        dwReserved: DWORD,
    ) -> HRESULT, 
    fn NotifyResume(
        dwReserved: DWORD,
    ) -> HRESULT,
}}

ENUM!{enum __MIDL___MIDL_itf_mscoree_0000_0013_0001
{
    eNoChecks	= 0,
    eSynchronization	= 0x1,
    eSharedState	= 0x2,
    eExternalProcessMgmt	= 0x4,
    eSelfAffectingProcessMgmt	= 0x8,
    eExternalThreading	= 0x10,
    eSelfAffectingThreading	= 0x20,
    eSecurityInfrastructure	= 0x40,
    eUI	= 0x80,
    eMayLeakOnAbort	= 0x100,
    eAll	= 0x1ff,
}}
pub type EApiCategories = __MIDL___MIDL_itf_mscoree_0000_0013_0001;

ENUM!{enum __MIDL___MIDL_itf_mscoree_0000_0013_0002
{
    eInitializeNewDomainFlags_None	= 0,
    eInitializeNewDomainFlags_NoSecurityChanges	= 0x2,
}}
pub type EInitializeNewDomainFlags = __MIDL___MIDL_itf_mscoree_0000_0013_0002;

RIDL!{#[uuid(0xB81FF171, 0x20F3, 0x11d2, 0x8D, 0xCC, 0x00, 0xA0, 0xC9, 0xB0, 0x05, 0x22)]
interface ITypeName(ITypeNameVtbl): IUnknown(IUnknownVtbl) {
    fn GetNameCount(
        pCount: *mut DWORD, 
    ) -> HRESULT,
    fn GetNames(
        count: DWORD, 
        rgbszNames: *mut BSTR, 
        pCount: *mut DWORD, 
    ) -> HRESULT,
    fn GetTypeArgumentCount(
        pCount: *mut DWORD,
    ) -> HRESULT, 
    fn GetTypeArguments(
        count: DWORD, 
        rgpArguments: *mut *mut ITypeName, 
        pCount: *mut DWORD, 
    ) -> HRESULT,
    fn GetModifierLength(
        pCount: *mut DWORD, 
    ) -> HRESULT,
    fn GetModifiers(
        count: DWORD, 
        rgModifiers: *mut DWORD, 
        pCount: *mut DWORD, 
    ) -> HRESULT, 
    fn GetAssemblyName(
        rgbszAssemblyNames: *mut BSTR, 
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xB81FF171, 0x20F3, 0x11d2, 0x8D, 0xCC, 0x00, 0xA0, 0xC9, 0xB0, 0x05, 0x23)]
interface ITypeNameBuilder(ITypeNameBuilderVtbl): IUnknown(IUnknownVtbl){
    fn OpenGenericArguments() -> HRESULT, 
    fn CloseGenericArguments() -> HRESULT,
    fn OpenGenericArgument() -> HRESULT, 
    fn CloseGenericArgument() -> HRESULT, 
    fn AddName(
        szName: LPCWSTR, 
    ) -> HRESULT, 
    fn AddPointer() -> HRESULT, 
    fn AddByRef() -> HRESULT, 
    fn AddSzArray() -> HRESULT, 
    fn AddArray(
        rank: DWORD,
    ) -> HRESULT, 
    fn AddAssemblySpec(
        szAssemblySpec: LPCWSTR,
    ) -> HRESULT,
    fn ToString_(
        pszStringRepresentation: *mut BSTR, 
    ) -> HRESULT, 
    fn Clear() -> HRESULT,
}}

RIDL!{#[uuid(0xB81FF171, 0x20F3, 0x11d2, 0x8D, 0xCC, 0x00, 0xA0, 0xC9, 0xB0, 0x05, 0x21)]
interface ITypeNameFactory(ITypeNameFactoryVtbl): IUnknown(IUnknownVtbl){
    fn ParseTypeName(
        szName: LPCWSTR, 
        pError: *mut DWORD, 
        ppTypeName: *mut *mut ITypeName,
    ) -> HRESULT, 
    fn GetTypeNameBuilder(
        ppTypeBuilder: *mut *mut ITypeNameBuilder,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xC3FCC19E, 0xA970, 0x11d2, 0x8B, 0x5A, 0x00, 0xA0, 0xC9, 0xB7, 0xC9, 0xC4)]
interface IManagedObject(IManagedObjectVtbl): IUnknown(IUnknownVtbl){
    fn GetSerializedBuffer(
        pBSTR: *mut BSTR,
    ) -> HRESULT, 
    fn GetObjectIdentity(
        pBSTRGUID: *mut BSTR, 
        AppDomainID: *mut i32, 
        pCCW: *mut i32,
    ) -> HRESULT,
}}

ENUM!{enum __MIDL___MIDL_itf_mscoree_0000_0014_0001
{
    eCurrentContext	= 0,
    eRestrictedContext	= 0x1,
}}

RIDL!{#[uuid(0xc62de18c, 0x2e23, 0x4aea, 0x84, 0x23, 0xb4, 0x0c, 0x1f, 0xc5, 0x9e, 0xae)]
interface ICLRAppDomainResourceMonitor(ICLRAppDomainResourceMonitorVtbl): IUnknown(IUnknownVtbl){
    fn GetCurrentAllocated(
        dwAppDomainId: DWORD, 
        pBytesAllocated: *mut ULONGLONG, 
    ) -> HRESULT, 
    fn GetCurrentSurvived(
        dwAppDomainId: DWORD, 
        pAppDomainBytesSurvived: *mut ULONGLONG, 
        pTotalBytesSurvived: *mut ULONGLONG, 
    ) -> HRESULT,
    fn GetCurrentCpuTime(
        dwAppDomainId: DWORD, 
        pMilliseconds: *mut ULONGLONG, 
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xCB2F6722, 0xAB3A, 0x11d2, 0x9C, 0x40, 0x00, 0xC0, 0x4F, 0xA3, 0x0A, 0x3E)]
interface ICorRuntimeHost(ICorRuntimeHostVtbl): IUnknown(IUnknownVtbl){
    /*virtual HRESULT STDMETHODCALLTYPE CreateLogicalThreadState( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteLogicalThreadState( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SwitchInLogicalThreadState( 
            /* [in] */ DWORD *pFiberCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SwitchOutLogicalThreadState( 
            /* [out] */ DWORD **pFiberCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LocksHeldByLogicalThread( 
            /* [out] */ DWORD *pCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MapFile( 
            /* [in] */ HANDLE hFile,
            /* [out] */ HMODULE *hMapAddress) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConfiguration( 
            /* [out] */ ICorConfiguration **pConfiguration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Start( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateDomain( 
            /* [in] */ LPCWSTR pwzFriendlyName,
            /* [in] */ IUnknown *pIdentityArray,
            /* [out] */ IUnknown **pAppDomain) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDefaultDomain( 
            /* [out] */ IUnknown **pAppDomain) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumDomains( 
            /* [out] */ HDOMAINENUM *hEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NextDomain( 
            /* [in] */ HDOMAINENUM hEnum,
            /* [out] */ IUnknown **pAppDomain) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CloseEnum( 
            /* [in] */ HDOMAINENUM hEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateDomainEx( 
            /* [in] */ LPCWSTR pwzFriendlyName,
            /* [in] */ IUnknown *pSetup,
            /* [in] */ IUnknown *pEvidence,
            /* [out] */ IUnknown **pAppDomain) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateDomainSetup( 
            /* [out] */ IUnknown **pAppDomainSetup) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateEvidence( 
            /* [out] */ IUnknown **pEvidence) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnloadDomain( 
            /* [in] */ IUnknown *pAppDomain) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CurrentDomain( 
            /* [out] */ IUnknown **pAppDomain) = 0;*/
}}