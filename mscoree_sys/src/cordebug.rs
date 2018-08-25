#![allow(dead_code, non_upper_case_globals, non_camel_case_types, non_snake_case)]
// cordebug.rs - MIT License
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
use winapi::shared::guiddef::GUID;
use winapi::shared::basetsd::{UINT64, ULONG32,ULONG64};
use winapi::shared::minwindef::{BOOL, DWORD};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use corhdr::mdMethodDef;

pub type HPROCESS = *mut c_void;
pub type HTHREAD = *mut c_void;
pub type TASKID = UINT64;
pub type CONNID = DWORD;

STRUCT!{struct _COR_IL_MAP
{
    oldOffset: ULONG32,
    newOffset: ULONG32,
    fAccurate: BOOL,
}}
pub type COR_IL_MAP = _COR_IL_MAP;

SIGNED_ENUM!{enum CorDebugIlToNativeMappingTypes
{
    NO_MAPPING	= -1,
    PROLOG	= -2,
    EPILOG	= -3,
}}

STRUCT!{struct COR_DEBUG_IL_TO_NATIVE_MAP{
    ilOffset: ULONG32, 
    nativeStartOffset: ULONG32, 
    nativeEndOffset: ULONG32, 
}}

ENUM!{enum CorDebugJITCompilerFlags
{
    CORDEBUG_JIT_DEFAULT	= 0x1,
    CORDEBUG_JIT_DISABLE_OPTIMIZATION	= 0x3,
    CORDEBUG_JIT_ENABLE_ENC	= 0x7,
}}

ENUM!{enum CorDebugJITCompilerFlagsDecprecated
{
    CORDEBUG_JIT_TRACK_DEBUG_INFO	= 0x1,
}}

ENUM!{enum CorDebugNGENPolicy
{
    DISABLE_LOCAL_NIC	= 1,
}}

pub type CORDB_ADDRESS = ULONG64;
pub type CORDB_REGISTER = ULONG64;
pub type CORDB_CONTINUE_STATUS = DWORD;

ENUM!{enum CorDebugBlockingReason
{
    BLOCKING_NONE	= 0,
    BLOCKING_MONITOR_CRITICAL_SECTION	= 0x1,
    BLOCKING_MONITOR_EVENT	= 0x2,
}}

STRUCT!{struct CorDebugBlockingObject {
    pBlockingObject: *mut ICorDebugValue, 
    dwTimeOut: DWORD, 
    blockingReason: CorDebugBlockingReason,
}}

STRUCT!{struct CorDebugExceptionObjectStackFrame{
    pModule: *mut ICorDebugModule, 
    ip: CORDB_ADDRESS, 
    methodDef: mdMethodDef, 
    isLastForeignExceptionFrame: BOOL, 
}}

STRUCT!{struct CorDebugGuidToTypeMapping{
    iid: GUID, 
    pType: *mut ICorDebugType,
}}

ENUM!{enum CorDebugPlatform
{
    CORDB_PLATFORM_WINDOWS_X86	= 0,
    CORDB_PLATFORM_WINDOWS_AMD64	= ( CORDB_PLATFORM_WINDOWS_X86 + 1 ) ,
    CORDB_PLATFORM_WINDOWS_IA64	= ( CORDB_PLATFORM_WINDOWS_AMD64 + 1 ) ,
    CORDB_PLATFORM_MAC_PPC	= ( CORDB_PLATFORM_WINDOWS_IA64 + 1 ) ,
    CORDB_PLATFORM_MAC_X86	= ( CORDB_PLATFORM_MAC_PPC + 1 ) ,
    CORDB_PLATFORM_WINDOWS_ARM	= ( CORDB_PLATFORM_MAC_X86 + 1 ) ,
    CORDB_PLATFORM_MAC_AMD64	= ( CORDB_PLATFORM_WINDOWS_ARM + 1 ) ,
    CORDB_PLATFORM_WINDOWS_ARM64	= ( CORDB_PLATFORM_MAC_AMD64 + 1 ), 
}}
//ICorDebugDataTarget
//ICorDebugStaticFieldSymbol
//ICorDebugInstanceFieldSymbol
//ICorDebugVariableSymbol
//ICorDebugMemoryBuffer
//ICorDebugMergedAssemblyRecord
//ICorDebugSymbolProvider
//ICorDebugSymbolProvider2
//ICorDebugDataTarget2
//ICorDebugLoadedModule
//ICorDebugDataTarget3
//ICorDebugMutableDataTarget
//ICorDebugMetaDataLocator
//enum CorDebugStepReason
//enum LoggingLevelEnum
//enum LogSwitchCallReason
//ICorDebugManagedCallback
//ICorDebugManagedCallback3
//enum CorDebugExceptionCallbackType
//enum CorDebugExceptionFlags
//enum CorDebugExceptionUnwindCallbackType
//ICorDebugManagedCallback2
//ICorDebugUnmanagedCallback
//enum enum CorDebugCreateProcessFlags
//enum CorDebugHandleType
//ICorDebug
//ICorDebugRemoteTarget
//ICorDebugRemote
//struct _COR_VERSION
//enum CorDebugInterfaceVersion
//ICorDebug2
//enum CorDebugThreadState
//ICorDebugController
//ICorDebugAppDomain
//ICorDebugAppDomain2
//ICorDebugEnum
//ICorDebugGuidToTypeEnum
//ICorDebugAppDomain3
//ICorDebugAppDomain4
//ICorDebugAssembly
//ICorDebugAssembly2
//ICorDebugAssembly3
//struct COR_TYPEID
//struct _COR_HEAPOBJECT
//ICorDebugHeapEnum
//enum CorDebugGenerationTypes
// struct _COR_SEGMENT
//enum CorDebugGCType
//struct _COR_HEAPINFO
//ICorDebugHeapSegmentEnum
//enum CorGCReferenceType
//struct COR_GC_REFERENCE
//ICorDebugGCReferenceEnum
//struct COR_ARRAY_LAYOUT
//struct COR_TYPE_LAYOUT
//struct COR_FIELD
//ICorDebugProcess
//ICorDebugProcess2
//ICorDebugProcess3
//ICorDebugProcess5
//enum CorDebugRecordFormat
//enum CorDebugDecodeEventFlagsWindows
//enum CorDebugDebugEventKind
//enum CorDebugStateChange
//ICorDebugDebugEvent
//enum CorDebugCodeInvokeKind
//enum CorDebugCodeInvokePurpose
//ICorDebugProcess6
//enum WriteableMetadataUpdateMode
//ICorDebugProcess7
//ICorDebugProcess8
//ICorDebugModuleDebugEvent
//ICorDebugExceptionDebugEvent
//ICorDebugBreakpoint
//ICorDebugFunctionBreakpoint
//ICorDebugModuleBreakpoint
//ICorDebugValueBreakpoint
//enum CorDebugIntercept
//enum CorDebugUnmappedStop
//struct COR_DEBUG_STEP_RANGE
//ICorDebugStepper
//ICorDebugStepper2
//enum CorDebugRegister
//ICorDebugRegisterSet
//ICorDebugRegisterSet2
//enum CorDebugUserState
//ICorDebugThread
//struct _COR_ACTIVE_FUNCTION
//ICorDebugThread2
//ICorDebugThread3
//ICorDebugThread4
//enum CorDebugSetContextFlag
//ICorDebugStackWalk
//enum CorDebugChainReason
//ICorDebugChain
//ICorDebugFrame
//enum CorDebugInternalFrameType
//ICorDebugInternalFrame
//ICorDebugInternalFrame2
//enum CorDebugMappingResult
//ICorDebugILFrame
//ICorDebugILFrame2
//ICorDebugILFrame3
//enum ILCodeKind
//ICorDebugILFrame4
//ICorDebugNativeFrame
//ICorDebugNativeFrame2
//ICorDebugNativeFrame3
//ICorDebugRuntimeUnwindableFrame

RIDL!{#[uuid(0xdba2d8c1, 0xe5c5, 0x4069, 0x8c, 0x13, 0x10, 0xa7, 0xc6, 0xab, 0xf4, 0x3d)]
interface ICorDebugModule(ICorDebugModuleVtbl): IUnknown(IUnknownVtbl){

}}
//ICorDebugModule2
//ICorDebugFunction
//ICorDebugFunction2
//ICorDebugFunction3
//ICorDebugCode
//struct _CodeChunkInfo
//ICorDebugCode2
//ICorDebugCode3
//struct _CorDebugEHClause
//ICorDebugILCode2
//ICorDebugClass
//ICorDebugClass2
//ICorDebugEval
//ICorDebugEval2
RIDL!{#[uuid(0xCC7BCAF7, 0x8A68, 0x11d2, 0x98, 0x3C, 0x00, 0x00, 0xF8, 0x08, 0x34, 0x2D)]
interface ICorDebugValue(ICorDebugValueVtbl): IUnknown(IUnknownVtbl){

}}
//ICorDebugValue2
//ICorDebugValue3
//ICorDebugGenericValue
//ICorDebugReferenceValue
//ICorDebugHeapValue
//ICorDebugHeapValue2
//ICorDebugHeapValue3
//ICorDebugObjectValue
//ICorDebugObjectValue2
//ICorDebugBoxValue
//ICorDebugStringValue
//ICorDebugArrayValue
//ICorDebugHandleValue
//ICorDebugContext
//ICorDebugComObjectValue
//ICorDebugObjectEnum
//ICorDebugBreakpointEnum
//ICorDebugStepperEnum
//ICorDebugProcessEnum
//ICorDebugThreadEnum
//ICorDebugFrameEnum
//ICorDebugChainEnum
//ICorDebugModuleEnum
//ICorDebugValueEnum
//ICorDebugCodeEnum
//ICorDebugTypeEnum
RIDL!{#[uuid(0xD613F0BB, 0xACE1, 0x4c19, 0xBD, 0x72, 0xE4, 0xC0, 0x8D, 0x5D, 0xA7, 0xF5)]
interface ICorDebugType(ICorDebugTypeVtbl): IUnknown(IUnknownVtbl){

}}
//ICorDebugErrorInfoEnum
//ICorDebugAppDomainEnum
//ICorDebugAssemblyEnum
//ICorDebugBlockingObjectEnum
//enum CorDebugMDAFlags
//ICorDebugMDA
//ICorDebugEditAndContinueErrorInfo
//ICorDebugEditAndContinueSnapshot
//ICorDebugExceptionObjectCallStackEnum
//ICorDebugExceptionObjectValue