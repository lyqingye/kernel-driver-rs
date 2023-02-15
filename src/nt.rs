#![allow(non_snake_case)]

use winapi::shared::ntdef::LIST_ENTRY;
use winapi::shared::ntdef::UNICODE_STRING;
#[link(name = "ntoskrnl")]
extern "system" {
    pub fn KeBugCheck(bug_check_code: u32) -> !;
}

#[repr(C)]
pub struct KLDR_DATA_TABLE_ENTRY {
    pub InLoadOrderLinks: LIST_ENTRY,
    pub ExceptionTable: usize,
    pub ExceptionTableSize: u32,
    pub GpValue: usize,
    pub NonPagedDebugInfo: usize,
    pub DllBase: usize,
    pub EntryPoint: usize,
    pub SizeOfImage: usize,
    pub FullDllName: UNICODE_STRING,
    pub BaseDllName: UNICODE_STRING,
    pub Flags: u32,
    pub LoadCount: u16,
    pub __Unused5: u16,
    pub SectionPointer: u32,
    pub CheckSum: u32,
}
