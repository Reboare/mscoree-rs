#![allow(dead_code, non_upper_case_globals, non_camel_case_types, non_snake_case)]

use winapi::shared::basetsd::SIZE_T;
use winapi::shared::minwindef::{ULONG};

STRUCT!{ struct _COR_GC_STATS {
    Flags: ULONG,   
    ExplicitGCCount: SIZE_T,
    GenCollectionsTaken: [SIZE_T; 3],
    CommittedKBytes: SIZE_T, 
    ReservedKBytes: SIZE_T,
    Gen0HeapSizeKBytes: SIZE_T,
    Gen1HeapSizeKBytes: SIZE_T,
    Gen2HeapSizeKBytes: SIZE_T,
    LargeObjectHeapSizeKBytes: SIZE_T,
    KBytesPromotedFromGen0: SIZE_T,
    KBytesPromotedFromGen1: SIZE_T,
}}
pub type COR_GC_STATS = _COR_GC_STATS;