#![allow(dead_code, non_upper_case_globals, non_camel_case_types, non_snake_case)]
// corhdr.rs - MIT License
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

use winapi::shared::basetsd::{UINT32};
use winapi::shared::minwindef::LPVOID;

pub type mdScope = LPVOID;
pub type mdToken = UINT32;
pub type mdTypeRef = mdToken;
pub type mdTypeDef = mdToken;
pub type mdFieldDef = mdToken;
pub type mdMethodDef = mdToken;
pub type mdParamDef = mdToken;
pub type mdInterfaceImpl = mdToken;
pub type mdMemberRef = mdToken;
pub type mdCustomAttribute = mdToken;
pub type mdPermission = mdToken;
pub type mdSignature = mdToken;
pub type mdEvent = mdToken;
pub type mdProperty = mdToken;
pub type mdModuleRef = mdToken;
pub type mdAssembly = mdToken;
pub type mdAssemblyRef = mdToken;
pub type mdFile = mdToken;
pub type mdExportedType = mdToken;
pub type mdManifestResource = mdToken;
pub type mdTypeSpec = mdToken;
pub type mdGenericParam = mdToken;
pub type mdMethodSpec = mdToken;
pub type mdGenericParamConstraint = mdToken;
pub type mdString = mdToken;
pub type mdCPToken = mdToken;


ENUM!{enum CorSaveSize
{
    cssAccurate             = 0x0000,
    cssQuick                = 0x0001,
    cssDiscardTransientCAs  = 0x0002,
}}

