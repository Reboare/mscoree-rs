#![allow(dead_code, non_upper_case_globals, non_camel_case_types, non_snake_case)]
// strongname.rs - MIT License
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
use winapi::shared::minwindef::{BYTE, DWORD, UINT, ULONG};
use winapi::shared::ntdef::{BOOLEAN, HANDLE, LPCSTR, LPCWSTR};

STRUCT!{struct PublicKeyBlob{
    SigAlgID: UINT,       // (ALG_ID) signature algorithm used to create the signature
    HashAlgID: UINT,      // (ALG_ID) hash algorithm used to create the signature
    cbPublicKey: ULONG,    // length of the key in bytes
    PublicKey: [BYTE;1],   // variable length byte array containing the key value in format output by CryptoAPI
}}

STDAPI!{#[deprecated] fn StrongNameErrorInfo() -> DWORD}
STDAPI!{#[deprecated] fn StrongNameFreeBuffer(
    pbMemory: *mut BYTE,
) -> c_void}
STDAPI!{#[deprecated] fn StrongNameKeyGen(
    wszKeyContainer: LPCWSTR, 
    dwFlags: DWORD, 
    ppbKeyBlob: *mut *mut BYTE, 
    pcbKeyBlob: *mut ULONG,
) -> bool}
STDAPI!{#[deprecated] fn StrongNameKeyGenEx(
    wszKeyContainer: LPCWSTR, 
    dwFlags: DWORD, 
    dwKeySize: DWORD, 
    ppbKeyBlob: *mut *mut BYTE, 
    pcbKeyBlob: *mut ULONG, 
) -> bool}
STDAPI!{#[deprecated] fn StrongNameKeyInstall(
    wszKeyContainer: LPCWSTR, 
    pbKeyBlob: *mut BYTE, 
    cbKeyBlob: ULONG,
) -> bool}
STDAPI!{#[deprecated] fn StrongNameKeyDelete(
    wszKeyContainer: LPCWSTR,
) -> bool}
STDAPI!{#[deprecated] fn StrongNameGetPublicKey(
    wszKeyContainer: LPCWSTR, 
    pbKeyBlob: *mut BYTE, 
    cbKeyBlob: ULONG, 
    ppbPublicKeyBlob: *mut *mut BYTE, 
    pcbPublicKeyBlob: *mut ULONG, 
) -> bool}
STDAPI!{#[deprecated] fn StrongNameGetPublicKeyEx(
    wszKeyContainer: LPCWSTR, 
    pbKeyBlob: *mut BYTE, 
    cbKeyBlob: ULONG, 
    ppbPublicKeyBlob: *mut *mut BYTE, 
    pcbPublicKeyBlob: *mut ULONG, 
    uHashAlgId: ULONG,
    uReserved: ULONG,
) -> bool}
STDAPI!{#[deprecated] fn StrongNameSignatureGeneration(
    wszFilePath: LPCWSTR, 
    wszKeyContainer: LPCWSTR, 
    pbKeyBlob: *mut BYTE, 
    cbKeyBlob: ULONG, 
    ppbSignatureBlob: *mut *mut BYTE, 
    pcbSignatureBlob: *mut ULONG, 
) -> bool}
STDAPI!{#[deprecated] fn StrongNameSignatureGenerationEx(
    wszFilePath: LPCWSTR, 
    wszKeyContainer: LPCWSTR, 
    pbKeyBlob: *mut BYTE, 
    cbKeyBlob: ULONG, 
    ppbSignatureBlob: *mut *mut BYTE, 
    pcbSignatureBlob: *mut ULONG, 
    dwFlags: DWORD, 
) -> bool}
STDAPI!{#[deprecated] fn StrongNameDigestGenerate(
    wszFilePath: LPCWSTR, 
    ppbDigestBlob: *mut *mut BYTE, 
    pcbDigestBlob: *mut ULONG,
    dwFlags: DWORD, 
) -> bool}
STDAPI!{#[deprecated] fn StrongNameDigestSign(
    wszKeyContainer: LPCWSTR, 
    pbKeyBlob: *mut BYTE, 
    cbKeyBlob: ULONG, 
    pbDigestBlob: *mut BYTE, 
    cbDigestBlob: ULONG, 
    hashAlgId: DWORD, 
    ppbSignatureBlob: *mut *mut BYTE, 
    pcbSignatureBlob: *mut ULONG, 
    dwFlags: DWORD,
) -> bool}
STDAPI!{#[deprecated] fn StrongNameDigestEmbed(
    wszFilePath: LPCWSTR, 
    pbSignatureBlob: *mut BYTE, 
    cbSignatureBlob: ULONG,
) -> bool}
STDAPI!{#[deprecated] fn StrongNameTokenFromAssembly(
    wszFilePath: LPCWSTR, 
    ppbStrongNameToken: *mut *mut BYTE, 
    pcbStrongNameToken: *mut ULONG, 
) -> bool}
STDAPI!{#[deprecated] fn StrongNameTokenFromAssemblyEx(
    wszFilePath: LPCWSTR, 
    ppbStrongNameToken: *mut *mut BYTE, 
    pcbStrongNameToken: *mut ULONG, 
    ppbPublicKeyBlob: *mut *mut BYTE, 
    pcbPublicKeyBlob: *mut ULONG, 
) -> bool}
STDAPI!{#[deprecated] fn StrongNameTokenFromPublicKey(
    pbPublicKeyBlob: *mut BYTE, 
    cbPublicKeyBlob: ULONG, 
    ppbStrongNameToken: *mut *mut BYTE, 
    pcbStrongNameToken: *mut ULONG, 
) -> bool}
STDAPI!{#[deprecated] fn StrongNameSignatureVerification(
    wszFilePath: LPCWSTR, 
    dwInFlags: DWORD, 
    pdwOutFlags: *mut DWORD,
) -> bool}
STDAPI!{#[deprecated] fn StrongNameSignatureVerificationEx(
    wszFilePath: LPCWSTR, 
    fForceVerification: BOOLEAN,
    pfWasVerified: *mut BOOLEAN,
) -> bool}
STDAPI!{#[deprecated] fn StrongNameSignatureVerificationEx2(
    wszFilePath: LPCWSTR, 
    fForceVerification: BOOLEAN,
    pbEcmaPublicKey: *mut BYTE, 
    cbEcmaPublicKey: DWORD,
    pfWasVerified: *mut BOOLEAN,
) -> bool}
STDAPI!{#[deprecated] fn StrongNameSignatureVerificationFromImage(
    pbBase: *mut BYTE, 
    dwLength: DWORD, 
    dwInFlags: DWORD, 
    pdwOutFlags: *mut DWORD, 
) -> bool}
STDAPI!{#[deprecated] fn StrongNameCompareAssemblies(
    wszAssembly1: LPCWSTR, 
    wszAssembly2: LPCWSTR, 
    pdwResult: *mut DWORD, 
) -> bool}
STDAPI!{#[deprecated] fn StrongNameHashSize(
    ulHashAlg: ULONG, 
    pcbSize: *mut DWORD, 
) -> bool}
STDAPI!{#[deprecated] fn StrongNameSignatureSize(
    pbPublicKeyBlob: *mut BYTE, 
    cbPublicKeyBlob: ULONG, 
    pcbSize: *mut DWORD, 
) -> bool}
STDAPI!{#[deprecated] fn GetHashFromAssemblyFile(
    szFilePath: LPCSTR, 
    piHashAlg: *mut UINT, 
    pbHash: *mut BYTE, 
    cchHash: DWORD, 
    pchHash: *mut DWORD,
) -> DWORD}
STDAPI!{#[deprecated] fn GetHashFromAssemblyFileW(
    wszFilePath: LPCWSTR, 
    piHashAlg: *mut UINT, 
    pbHash: *mut BYTE, 
    cchHash: DWORD, 
    pchHash: *mut DWORD,
) -> DWORD}
STDAPI!{#[deprecated] fn GetHashFromFile(
    szFilePath: LPCSTR, 
    piHashAlg: *mut UINT, 
    pbHash: *mut BYTE, 
    cchHash: DWORD, 
    pchHash: *mut DWORD, 
) -> DWORD}
STDAPI!{#[deprecated] fn GetHashFromFileW(
    wszFilePath: LPCWSTR, 
    piHashAlg: *mut UINT, 
    pbHash: *mut BYTE, 
    cchHash: DWORD, 
    pchHash: *mut DWORD,
) -> DWORD}
STDAPI!{#[deprecated] fn GetHashFromHandle(
    hFile: HANDLE,
    piHashAlg: *mut UINT, 
    pbHash: *mut BYTE, 
    cchHash: DWORD, 
    pchHash: *mut DWORD, 
) -> DWORD}
STDAPI!{#[deprecated] fn GetHashFromBlob(
    pbBlob: *mut BYTE, 
    cchBlob: DWORD, 
    piHashAlg: *mut UINT, 
    pbHash: *mut BYTE, 
    cchHash: DWORD, 
    pchHash: *mut DWORD,
) -> DWORD}
STDAPI!{#[deprecated] fn StrongNameGetBlob(
    wszFilePath: LPCWSTR, 
    pbBlob: *mut BYTE, 
    pcbBlob: *mut DWORD, 
) -> bool}
STDAPI!{#[deprecated] fn StrongNameGetBlobFromImage(
    pbBase: *mut BYTE, 
    dwLength: DWORD, 
    pbBlob: *mut BYTE, 
    pcbBlob: *mut DWORD,
) -> bool}
