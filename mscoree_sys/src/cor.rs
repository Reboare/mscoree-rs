#![allow(dead_code, non_upper_case_globals, non_camel_case_types, non_snake_case)]
// cor.rs - MIT License
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

use winapi::ctypes::{c_int, c_short, c_void};
use winapi::shared::basetsd::ULONG32;
use winapi::shared::guiddef::{GUID, REFCLSID, REFIID};
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPCVOID, LPVOID, PBYTE, ULONG, USHORT};
use winapi::shared::ntdef::{LPCWSTR, LPWSTR, PVOID};
use winapi::shared::winerror::HRESULT;
use winapi::um::winnt::{IMAGE_SCN_MEM_READ, IMAGE_SCN_CNT_INITIALIZED_DATA,  IMAGE_SCN_MEM_WRITE,  IMAGE_SCN_CNT_CODE, IMAGE_SCN_MEM_EXECUTE,};
use winapi::um::objidlbase::IStream;
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use crate::corhdr::{CorSaveSize, mdToken, mdTypeDef};

DEFINE_GUID!(LIBID_ComPlusRuntime, 0xbed7f4ea, 0x1a96, 0x11d2, 0x8f, 0x8, 0x0, 0xa0, 0xc9, 0xa6, 0x18, 0x6d);
DEFINE_GUID!(GUID_ExportedFromComPlus, 0x90883f05, 0x3d28, 0x11d2, 0x8f, 0x17, 0x0, 0xa0, 0xc9, 0xa6, 0x18, 0x6d);
DEFINE_GUID!(GUID_ManagedName, 0xf21f359, 0xab84, 0x41e8, 0x9a, 0x78, 0x36, 0xd1, 0x10, 0xe6, 0xd2, 0xf9);
DEFINE_GUID!(GUID_Function2Getter, 0x54fc8f55, 0x38de, 0x4703, 0x9c, 0x4e, 0x25, 0x3, 0x51, 0x30, 0x2b, 0x1c);
DEFINE_GUID!(CLSID_CorMetaDataDispenserRuntime, 0x1ec2de53, 0x75cc, 0x11d2, 0x97, 0x75, 0x0, 0xa0, 0xc9, 0xb4, 0xd5, 0xc);
DEFINE_GUID!(GUID_DispIdOverride, 0xcd2bc5c9, 0xf452, 0x4326, 0xb7, 0x14, 0xf9, 0xc5, 0x39, 0xd4, 0xda, 0x58);
DEFINE_GUID!(GUID_ForceIEnumerable, 0xb64784eb, 0xd8d4, 0x4d9b, 0x9a, 0xcd, 0x0e, 0x30, 0x80, 0x64, 0x26, 0xf7);
DEFINE_GUID!(GUID_PropGetCA, 0x2941ff83, 0x88d8, 0x4f73, 0xb6, 0xa9, 0xbd, 0xf8, 0x71, 0x2d, 0x00, 0x0d);
DEFINE_GUID!(GUID_PropPutCA, 0x29533527, 0x3683, 0x4364, 0xab, 0xc0, 0xdb, 0x1a, 0xdd, 0x82, 0x2f, 0xa2);
DEFINE_GUID!(CLSID_CLR_v1_MetaData, 0x005023ca, 0x72b1, 0x11d3, 0x9f, 0xc4, 0x0, 0xc0, 0x4f, 0x79, 0xa0, 0xa3);
DEFINE_GUID!(CLSID_CLR_v2_MetaData, 0xefea471a, 0x44fd, 0x4862, 0x92, 0x92, 0xc, 0x58, 0xd4, 0x6e, 0x1f, 0x3a);
DEFINE_GUID!(MetaDataCheckDuplicatesFor, 0x30fe7be8, 0xd7d9, 0x11d2, 0x9f, 0x80, 0x0, 0xc0, 0x4f, 0x79, 0xa0, 0xa3);
DEFINE_GUID!(MetaDataRefToDefCheck, 0xde3856f8, 0xd7d9, 0x11d2, 0x9f, 0x80, 0x0, 0xc0, 0x4f, 0x79, 0xa0, 0xa3);
DEFINE_GUID!(MetaDataNotificationForTokenMovement, 0xe5d71a4c, 0xd7da, 0x11d2, 0x9f, 0x80, 0x0, 0xc0, 0x4f, 0x79, 0xa0, 0xa3);
DEFINE_GUID!(MetaDataSetUpdate, 0x2eee315c, 0xd7db, 0x11d2, 0x9f, 0x80, 0x0, 0xc0, 0x4f, 0x79, 0xa0, 0xa3);
DEFINE_GUID!(MetaDataImportOption, 0x79700f36, 0x4aac, 0x11d3, 0x84, 0xc3, 0x0, 0x90, 0x27, 0x86, 0x8c, 0xb1);
DEFINE_GUID!(MetaDataThreadSafetyOptions, 0xf7559806, 0xf266, 0x42ea, 0x8c, 0x63, 0xa, 0xdb, 0x45, 0xe8, 0xb2, 0x34);
DEFINE_GUID!(MetaDataErrorIfEmitOutOfOrder, 0x1547872d, 0xdc03, 0x11d2, 0x94, 0x20, 0x0, 0x0, 0xf8, 0x8, 0x34, 0x60);
DEFINE_GUID!(MetaDataGenerateTCEAdapters, 0xdcc9de90, 0x4151, 0x11d3, 0x88, 0xd6, 0x0, 0x90, 0x27, 0x54, 0xc4, 0x3a);
DEFINE_GUID!(MetaDataTypeLibImportNamespace, 0xf17ff889, 0x5a63, 0x11d3, 0x9f, 0xf2, 0x0, 0xc0, 0x4f, 0xf7, 0x43, 0x1a);
DEFINE_GUID!(MetaDataLinkerOptions, 0x47e099b6, 0xae7c, 0x4797, 0x83, 0x17, 0xb4, 0x8a, 0xa6, 0x45, 0xb8, 0xf9);
DEFINE_GUID!(MetaDataRuntimeVersion, 0x47e099b7, 0xae7c, 0x4797, 0x83, 0x17, 0xb4, 0x8a, 0xa6, 0x45, 0xb8, 0xf9);
DEFINE_GUID!(MetaDataMergerOptions, 0x132d3a6e, 0xb35d, 0x464e, 0x95, 0x1a, 0x42, 0xef, 0xb9, 0xfb, 0x66, 0x1);
DEFINE_GUID!(MetaDataPreserveLocalRefs, 0xa55c0354, 0xe91b, 0x468b, 0x86, 0x48, 0x7c, 0xc3, 0x10, 0x35, 0xd5, 0x33);

pub type UVCP_CONSTANT = *const c_void;

STDAPI!{fn _CorDllMain(
    hInst: HINSTANCE, 
    dwReason: DWORD, 
    lpReserved: LPVOID,
) -> BOOL}
STDAPI!{fn _CorExeMain() -> c_int}
STDAPI!{fn _CorExeMainInternal() -> c_int}
STDAPI!{fn _CorExeMain2(
    pUnmappedPE: PBYTE, 
    cUnmappedPE: DWORD, 
    pImageNameIn: LPWSTR, 
    pLoadersFileName: LPWSTR,
    pCmdLine: LPWSTR,
) -> c_int}
STDAPI!{fn _CorValidateImage(
    ImageBase: *mut PVOID,
    FileName: LPCWSTR,
) -> HRESULT}
STDAPI!{#[deprecated]fn CoInitializeEE(
    fFlags: DWORD,
) -> HRESULT}
STDAPI!{#[deprecated]fn CoUninitializeEE(
    fFlags: BOOL,
)}

STDAPI!{#[deprecated]fn CoEEShutDownCOM()}

ENUM!{enum tagCOINITCOR
{
    COINITCOR_DEFAULT       = 0x0,           
}}
pub type COINITICOR = tagCOINITCOR;

ENUM!{enum tagCOINITEE
{
    COINITEE_DEFAULT        = 0x0,          
    COINITEE_DLL            = 0x1,       
    COINITEE_MAIN           = 0x2,   
}}
pub type COINITIEE = tagCOINITEE;

ENUM!{enum tagCOUNINITEE
{
    COUNINITEE_DEFAULT      = 0x0,     
    COUNINITEE_DLL          = 0x1,      
}}
pub type COUNINITIEE = tagCOUNINITEE;

STDAPI!{#[deprecated]fn CoInitializeCor(
    fFlags: DWORD,
) -> HRESULT}

STDAPI!{#[deprecated]fn CoUninitializeCor()}

DEFINE_GUID!(CLSID_Cor, 0xbee00010, 0xee77, 0x11d0, 0xa0, 0x15, 0x00, 0xc0, 0x4f, 0xbb, 0xb8, 0x84);
DEFINE_GUID!(CLSID_CorMetaDataDispenser, 0xe5cb7a31, 0x7512, 0x11d2, 0x89, 0xce, 0x0, 0x80, 0xc7, 0x92, 0xe5, 0xd8);
DEFINE_GUID!(CLSID_CorMetaDataDispenserReg, 0x435755ff, 0x7397, 0x11d2, 0x97, 0x71, 0x0, 0xa0, 0xc9, 0xb4, 0xd5, 0xc);
DEFINE_GUID!(CLSID_CorMetaDataReg, 0x87f3a1f5, 0x7397, 0x11d2, 0x97, 0x71, 0x0, 0xa0, 0xc9, 0xb4, 0xd5, 0xc);
DEFINE_GUID!(IID_IMetaDataError, 0xb81ff171, 0x20f3, 0x11d2, 0x8d, 0xcc, 0x0, 0xa0, 0xc9, 0xb0, 0x9c, 0x19);

INTERFACE_BINDING!{interface IMetaDataError(IMetaDataErrorVtbl): IUnknown(IUnknownVtbl){
    fn OnError(
        hrError: HRESULT, 
        token: ULONG32,
    ) -> HRESULT,
}}

DEFINE_GUID!(IID_IMapToken, 0x6a3ea8b, 0x225, 0x11d1, 0xbf, 0x72, 0x0, 0xc0, 0x4f, 0xc3, 0x1e, 0x12);

INTERFACE_BINDING!{interface IMapToken(IMapTokenVtbl): IUnknown(IUnknownVtbl){
    fn Map(
        tkImp: ULONG32, 
        tkEmit: ULONG32,
    ) -> HRESULT, 
}}

DEFINE_GUID!(IID_IMetaDataDispenser, 0x809c652e, 0x7396, 0x11d2, 0x97, 0x71, 0x00, 0xa0, 0xc9, 0xb4, 0xd5, 0x0c);

INTERFACE_BINDING!{interface IMetaDataDispenser(IMetaDataDispenserVtbl): IUnknown(IUnknownVtbl){
    fn DefineScope(
        rclsid: REFCLSID, 
        dwCreateFlags: DWORD, 
        riid: REFIID, 
        ppIUnk: *mut *mut IUnknown, 
    ) -> HRESULT, 
    fn OpenScope(
        szScope: LPCWSTR, 
        dwOpenFlags: DWORD, 
        riid: REFIID, 
        ppIUnk: *mut *mut IUnknown, 
    ) -> HRESULT, 
    fn OpenScopeOnMemory(
        pData: LPCVOID, 
        cbData: ULONG, 
        dwOpenFlags: DWORD, 
        riid: REFIID, 
        ppIUnk: *mut *mut IUnknown,
    ) -> HRESULT,
}}

DEFINE_GUID!(IID_IMetaDataEmit, 0xba3fee4c, 0xecb9, 0x4e41, 0x83, 0xb7, 0x18, 0x3f, 0xa4, 0x1c, 0xd8, 0x59);
INTERFACE_BINDING!{interface IMetaDataEmit(IMetaDataEmitVtbl): IUnknown(IUnknownVtbl){
    fn SetModuleProps(
        szName: LPCWSTR,
    ) -> HRESULT,
    fn Save(
        szFile: LPCWSTR, 
        dwSaveFlags: DWORD, 
    ) -> HRESULT, 
    fn SaveToStream(
        pIStream: *mut IStream,
        dwSaveFlags: DWORD, 
    ) -> HRESULT, 
    fn GetSaveSize(
        fSave: CorSaveSize, 
        pdwSaveSize: *mut DWORD,
    ) -> HRESULT,
    fn DefineTypeDef(
        szTypeDef: LPCWSTR, 
        dwTypeDefFlags: DWORD, 
        tkExtends: mdToken, 
        rtkImplements: *mut mdToken,
        tdEncloser: mdTypeDef, 
        ptd: *mut mdTypeDef,
    ) -> HRESULT,
    fn DefineNestedType() -> HRESULT, 
    fn SetHandler() -> HRESULT,
    fn DefineMethod() -> HRESULT, 
    fn DefineMethodImpl() -> HRESULT,
    fn DefineTypeRefByName() -> HRESULT, 
    fn DefineImportType() -> HRESULT,
    fn DefineMemberRef() -> HRESULT,
    fn DefineImportMember() -> HRESULT,
    fn DefineEvent() -> HRESULT, 
    fn SetClassLayout() -> HRESULT, 
    fn DeleteClassLayout() -> HRESULT, 
    fn SetFieldMarshal() -> HRESULT, 
    fn DeleteFieldMarshal() -> HRESULT, 
    fn DefinePermissionSet() -> HRESULT, 
    fn SetRVA() -> HRESULT, 
    fn GetTokenFromSig() -> HRESULT, 
    fn DefineModuleRef() -> HRESULT, 
    fn SetParent() -> HRESULT, 
    fn GetTokenFromTypeSpec() -> HRESULT, 
    fn SaveToMemory() -> HRESULT, 
    fn DefineUserString() -> HRESULT, 
    fn DeleteToken() -> HRESULT, 
    fn SetMethodProps() -> HRESULT, 
    fn SetTypeDefProps() -> HRESULT, 
    fn SetEventProps() -> HRESULT, 
    fn SetPermissionSetProps() -> HRESULT, 
    fn DefinePinvokeMap() -> HRESULT, 
    fn SetPinvokeMap() -> HRESULT, 
    fn DeletePinvokeMap() -> HRESULT, 
    fn DefineCustomAttribute() -> HRESULT,
    fn SetCustomAttributeValue() -> HRESULT, 
    fn DefineField() -> HRESULT, 
    fn DefineProperty() -> HRESULT, 
    fn DefineParam() -> HRESULT, 
    fn SetFieldProps() -> HRESULT,
    fn SetPropertyProps() -> HRESULT, 
    fn SetParamProps() -> HRESULT, 
    fn DefineSecurityAttributeSet() -> HRESULT,
    fn ApplyEditAndContinue() -> HRESULT, 
    fn TranslateSigWithScope() -> HRESULT, 
    fn SetMethodImplFlags() -> HRESULT, 
    fn SetFieldRVA() -> HRESULT,
    fn Merge() -> HRESULT,
    fn MergeEnd() -> HRESULT,
}}

DEFINE_GUID!(IID_IMetaDataEmit2, 0xf5dd9950, 0xf693, 0x42e6, 0x83, 0xe, 0x7b, 0x83, 0x3e, 0x81, 0x46, 0xa9);
INTERFACE_BINDING!{interface IMetaDataEmit2(IMetaDataEmit2Vtbl): IMetaDataEmit(IMetaDataEmitVtbl){
    fn DefineMethodSpec() -> HRESULT, 
    fn GetDeltaSaveSize() -> HRESULT, 
    fn SaveDelta() -> HRESULT, 
    fn SaveDeltaToStream() -> HRESULT, 
    fn SaveDeltaToMemory() -> HRESULT, 
    fn DefineGenericParam() -> HRESULT, 
    fn SetGenericParamProps() -> HRESULT, 
    fn ResetENCLog() -> HRESULT,
}}

DEFINE_GUID!(IID_IMetaDataImport, 0x7dac8207, 0xd3ae, 0x4c75, 0x9b, 0x67, 0x92, 0x80, 0x1a, 0x49, 0x7d, 0x44);
INTERFACE_BINDING!{interface IMetaDataImport(IMetaDataImportVtbl): IUnknown(IUnknownVtbl){

}}

DEFINE_GUID!(IID_IMetaDataImport2, 0xfce5efa0, 0x8bba, 0x4f8e, 0xa0, 0x36, 0x8f, 0x20, 0x22, 0xb0, 0x84, 0x66);
INTERFACE_BINDING!{interface IMetaDataImport2(IMetaDataImport2Vtbl): IMetaDataImport(IMetaDataImportVtbl){

}}

DEFINE_GUID!(IID_IMetaDataFilter, 0xd0e80dd1, 0x12d4, 0x11d3, 0xb3, 0x9d, 0x0, 0xc0, 0x4f, 0xf8, 0x17, 0x95);
INTERFACE_BINDING!{interface IMetaDataFilter(IMetaDataFilterVtbl): IUnknown(IUnknownVtbl){
    fn UnmarkAll() -> HRESULT, 
    fn MarkToken(
        tk: mdToken,
    ) -> HRESULT, 
    fn IsTokenMarked(
        tk: mdToken, 
        pIsMarked: *mut BOOL, 
    ) -> HRESULT, 
}}

DEFINE_GUID!(IID_IHostFilter, 0xd0e80dd3, 0x12d4, 0x11d3, 0xb3, 0x9d, 0x0, 0xc0, 0x4f, 0xf8, 0x17, 0x95);
INTERFACE_BINDING!{interface IHostFilter(IHostFilterVtbl): IUnknown(IUnknownVtbl){
    fn MarkToken(
        tk: mdToken,
    ) -> HRESULT,
}}

STRUCT!{struct OSINFO
{
    dwOSPlatformId: DWORD,
    dwOSMajorVersion: DWORD,
    dwOSMinorVersion: DWORD,
}}

STRUCT!{struct ASSEMBLYMETADATA
{
    usMajorVersion: USHORT,
    usMinorVersion: USHORT,
    usBuildNumber: USHORT,
    usRevisionNumber: USHORT,
    szLocale: LPCWSTR,
    cbLocale: ULONG,
    rProcessor: *mut DWORD,
    ulProcessor: ULONG,
    rOS: *mut OSINFO,
    ulOS: ULONG,
}}

DEFINE_GUID!(IID_IMetaDataAssemblyEmit, 0x211ef15b, 0x5317, 0x4438, 0xb1, 0x96, 0xde, 0xc8, 0x7b, 0x88, 0x76, 0x93);
INTERFACE_BINDING!{interface IMetaDataAssemblyEmit(IMetaDataAssemblyEmitVtbl): IUnknown(IUnknownVtbl){

}}

DEFINE_GUID!(IID_IMetaDataAssemblyImport, 0xee62470b, 0xe94b, 0x424e, 0x9b, 0x7c, 0x2f, 0x0, 0xc9, 0x24, 0x9f, 0x93);
INTERFACE_BINDING!{interface IMetaDataAssemblyImport(IMetaDataAssemblyImportVtbl): IUnknown(IUnknownVtbl){

}}

ENUM!{enum CorValidatorModuleType
{
    ValidatorModuleTypeInvalid      = 0x0,
    ValidatorModuleTypeMin          = 0x00000001,
    ValidatorModuleTypePE           = 0x00000001,
    ValidatorModuleTypeObj          = 0x00000002,
    ValidatorModuleTypeEnc          = 0x00000003,
    ValidatorModuleTypeIncr         = 0x00000004,
    ValidatorModuleTypeMax          = 0x00000004,
}}

DEFINE_GUID!(IID_IMetaDataValidate, 0x4709c9c6, 0x81ff, 0x11d3, 0x9f, 0xc7, 0x0, 0xc0, 0x4f, 0x79, 0xa0, 0xa3);
INTERFACE_BINDING!{interface IMetaDataValidate(IMetaDataValidateVtbl): IUnknown(IUnknownVtbl){
    fn ValidatorInit(
        dwModuleType: DWORD, 
    ) -> HRESULT, 
    fn ValidateMetaData() -> HRESULT, 
}}

DEFINE_GUID!(IID_IMetaDataDispenserEx, 0x31bcfce2, 0xdafb, 0x11d2, 0x9f, 0x81, 0x0, 0xc0, 0x4f, 0x79, 0xa0, 0xa3);
INTERFACE_BINDING!{interface IMetaDataDispenserEx(IMetaDataDispenserExVtbl): IMetaDataDispenser(IMetaDataDispenserVtbl){

}}

ENUM!{enum CorRegFlags
{
    regNoCopy = 0x00000001,
    regConfig = 0x00000002,
    regHasRefs = 0x00000004,
}}

pub type CVID = GUID;

STRUCT!{struct CVStruct {
    Major: c_short,
    Minor: c_short,
    Sub: c_short,
    Build: c_short,
}}

pub type HCEESECTION = *mut c_void;

ENUM!{enum CeeSectionAttr {
    sdNone =        0,
    sdReadOnly =    IMAGE_SCN_MEM_READ | IMAGE_SCN_CNT_INITIALIZED_DATA,
    sdReadWrite =   sdReadOnly | IMAGE_SCN_MEM_WRITE,
    sdExecute =     IMAGE_SCN_MEM_READ | IMAGE_SCN_CNT_CODE | IMAGE_SCN_MEM_EXECUTE,
}}

ENUM!{enum CeeSectionRelocType {
    // generate only a section-relative reloc, nothing into .reloc section
    srRelocAbsolute,

    // generate a .reloc for a pointer sized location, 
    // This is transformed into BASED_HIGHLOW or BASED_DIR64 based on the platform
    srRelocHighLow      = 3,

    // generate a .reloc for the top 16-bits of a 32 bit number, where the
    // bottom 16 bits are included in the next word in the .reloc table
    srRelocHighAdj,     // Never Used

    // generate a token map relocation, nothing into .reloc section 
    srRelocMapToken,

    // relative address fixup
    srRelocRelative,

    // Generate only a section-relative reloc, nothing into .reloc
    // section.  This reloc is relative to the file position of the
    // section, not the section's virtual address.
    srRelocFilePos,

    // code relative address fixup
    srRelocCodeRelative,

    // generate a .reloc for a 64 bit address in an ia64 movl instruction 
    srRelocIA64Imm64,

    // generate a .reloc for a 64 bit address
    srRelocDir64,

    // generate a .reloc for a 25-bit PC relative address in an ia64 br.call instruction 
    srRelocIA64PcRel25,

    // generate a .reloc for a 64-bit PC relative address in an ia64 brl.call instruction 
    srRelocIA64PcRel64,

    // generate a 30-bit section-relative reloc, used for tagged pointer values
    srRelocAbsoluteTagged,


    // A sentinel value to help ensure any additions to this enum are reflected 
    // in PEWriter.cpp's RelocName array.
    srRelocSentinel,

    // Flags that can be used with the above reloc types

    // do not emit base reloc
    srNoBaseReloc = 0x4000,
    
    // pre-fixup contents of memory are ptr rather than a section offset
    srRelocPtr = 0x8000,

    // legal enums which include the Ptr flag
    srRelocAbsolutePtr  = srRelocPtr + srRelocAbsolute,
    srRelocHighLowPtr   = srRelocPtr + srRelocHighLow,
    srRelocRelativePtr  = srRelocPtr + srRelocRelative,
    srRelocIA64Imm64Ptr = srRelocPtr + srRelocIA64Imm64,
    srRelocDir64Ptr     = srRelocPtr + srRelocDir64,
}}

DEFINE_GUID!(IID_ICeeGen, 0x7ed1bdff, 0x8e36, 0x11d2, 0x9c, 0x56, 0x0, 0xa0, 0xc9, 0xb7, 0xcc, 0x45);
INTERFACE_BINDING!{interface ICeeGen(ICeeGenVtbl): IUnknown(IUnknownVtbl){

}}

//incomplete