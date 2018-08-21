#![allow(dead_code, non_camel_case_types, non_snake_case)]
// clrdata.rs - MIT License
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
use winapi::shared::basetsd::{ULONG32, ULONG64};
use winapi::shared::guiddef::{GUID, REFIID};
use winapi::shared::minwindef::{BYTE};
use winapi::shared::ntdef::{LPCWSTR};
use winapi::shared::winerror::HRESULT;
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

pub type CLRDATA_ADDRESS = ULONG64;

FUNC_PTR!{CLRDataCreateInstance(
    iid: REFIID, 
    target: *mut ICLRDataTarget, 
    iface: *mut *mut c_void 
) -> HRESULT}

RIDL!{#[uuid(0x3E11CCEE, 0xD08B, 0x43e5, 0xAF, 0x01, 0x32, 0x71, 0x7A, 0x64, 0xDA, 0x03)]
interface ICLRDataTarget(ICLRDataTargetVtbl): IUnknown(IUnknownVtbl){
    fn GetMachineType(
        machineType: *mut ULONG32,
    ) -> HRESULT,
    fn GetPointerSize(
        pointerSize: *mut ULONG32, 
    ) -> HRESULT,
    fn GetImageBase(
        imagePath: LPCWSTR, 
        baseAddress: *mut CLRDATA_ADDRESS, 
    ) -> HRESULT, 
    fn ReadVirtual(
        address: CLRDATA_ADDRESS, 
        buffer: *mut BYTE, 
        bytesRequested: ULONG32, 
        bytesRead: *mut ULONG32, 
    ) -> HRESULT, 
    fn WriteVirtual(
        address: CLRDATA_ADDRESS, 
        buffer: *mut BYTE, 
        bytesRequested: ULONG32, 
        bytesWritten: *mut ULONG32, 
    ) -> HRESULT, 
    fn GetTLSValue(
        threadID: ULONG32, 
        index: ULONG32, 
        value: *mut CLRDATA_ADDRESS, 
    ) -> HRESULT,
    fn SetTLSValue(
        threadID: ULONG32, 
        index: ULONG32, 
        value: CLRDATA_ADDRESS, 
    ) -> HRESULT,
    fn GetCurrentThreadID(
        threadID: *mut ULONG32, 
    ) -> HRESULT,
    fn GetThreadContext(
        threadID: ULONG32, 
        contextFlags: ULONG32, 
        contextSize: ULONG32, 
        context: *mut BYTE,
    ) -> HRESULT,
    fn SetThreadContext(
        threadID: ULONG32, 
        contextSize: ULONG32, 
        context: *mut BYTE,
    ) -> HRESULT, 
    fn Request(
        reqCode: ULONG32, 
        inBufferSize: ULONG32, 
        inBuffer: *mut BYTE, 
        outBufferSize: ULONG32, 
        outBuffer: *mut BYTE,
    ) -> HRESULT, 
}}

RIDL!{#[uuid(0x6d05fae3, 0x189c, 0x4630, 0xa6, 0xdc, 0x1c, 0x25, 0x1e, 0x1c, 0x01, 0xab)]
interface ICLRDataTarget2(ICLRDataTarget2Vtbl): ICLRDataTarget(ICLRDataTargetVtbl){
    fn AllocVirtual(
        addr: CLRDATA_ADDRESS, 
        size: ULONG32, 
        typeFlags: ULONG32, 
        protectFlags: ULONG32, 
        virt: *mut CLRDATA_ADDRESS, 
    ) -> HRESULT, 
    fn FreeVirtual(
        addr: CLRDATA_ADDRESS, 
        size: ULONG32, 
        typeFlags: ULONG32, 
    ) -> HRESULT, 
}}

RIDL!{#[uuid(0xa5664f95, 0x0af4, 0x4a1b, 0x96, 0x0e, 0x2f, 0x33, 0x46, 0xb4, 0x21, 0x4c)]
interface ICLRDataTarget3(ICLRDataTarget3Vtbl): ICLRDataTarget2(ICLRDataTarget2Vtbl){
    fn GetExceptionRecord(
        bufferSize: ULONG32, 
        bufferUsed: *mut ULONG32, 
        buffer: *mut BYTE, 
    ) -> HRESULT, 
    fn GetExceptionContextRecord(
        bufferSize: ULONG32, 
        bufferUsed: ULONG32, 
        buffer: *mut BYTE,
    ) -> HRESULT, 
    fn GetExceptionThreadID(
        threadID: *mut ULONG32, 
    ) -> HRESULT, 
}}

RIDL!{#[uuid(0xaa8fa804, 0xbc05, 0x4642, 0xb2, 0xc5, 0xc3, 0x53, 0xed, 0x22, 0xfc, 0x63)]
interface ICLRMetadataLocator(ICLRMetadataLocatorVtbl): IUnknown(IUnknownVtbl){
    fn GetMetadata(
        imagePath: LPCWSTR, 
        imageTimestamp: ULONG32, 
        imageSize: ULONG32, 
        mvid: GUID, 
        mdRva: ULONG32, 
        flags: ULONG32, 
        bufferSize: ULONG32, 
        buffer: *mut BYTE,
        dataSize: *mut ULONG32, 
    ) -> HRESULT, 
}}

RIDL!{#[uuid(0xBCDD6908, 0xBA2D, 0x4ec5, 0x96, 0xCF, 0xDF, 0x4D, 0x5C, 0xDC, 0xB4, 0xA4)]
interface ICLRDataEnumMemoryRegionsCallback(ICLRDataEnumMemoryRegionsCallbackVtbl): IUnknown(IUnknownVtbl){
    fn EnumMemoryRegion(
        address: CLRDATA_ADDRESS, 
        size: ULONG32, 
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x3721A26F, 0x8B91, 0x4D98, 0xA3, 0x88, 0xDB, 0x17, 0xB3, 0x56, 0xFA, 0xDB)]
interface ICLRDataEnumMemoryRegionsCallback2(ICLRDataEnumMemoryRegionsCallback2Vtbl): ICLRDataEnumMemoryRegionsCallback(ICLRDataEnumMemoryRegionsCallbackVtbl){
    fn UpdateMemoryRegion(
        address: CLRDATA_ADDRESS, 
        size: ULONG32, 
        buffer: *mut BYTE,
    ) -> HRESULT,
}}

ENUM!{enum CLRDataEnumMemoryFlags
{
    CLRDATA_ENUM_MEM_DEFAULT	= 0,
    CLRDATA_ENUM_MEM_MINI	= CLRDATA_ENUM_MEM_DEFAULT,
    CLRDATA_ENUM_MEM_HEAP	= 0x1,
    CLRDATA_ENUM_MEM_TRIAGE	= 0x2,
}}

RIDL!{#[uuid(0x471c35b4, 0x7c2f, 0x4ef0, 0xa9, 0x45, 0x00, 0xf8, 0xc3, 0x80, 0x56, 0xf1)]
interface ICLRDataEnumMemoryRegions(ICLRDataEnumMemoryRegionsVtbl): IUnknown(IUnknownVtbl){
    fn EnumMemoryRegion(
        callback: *mut ICLRDataEnumMemoryRegionsCallback, 
        miniDumpFlags: ULONG32, 
        clrFlags: CLRDataEnumMemoryFlags,
    ) -> HRESULT,
}}