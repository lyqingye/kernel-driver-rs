use alloc::string::String;
use winapi::{km::wdm::DRIVER_OBJECT, shared::ntdef::LIST_ENTRY};

use crate::{lang::unicode_string_to_string, nt::KLDR_DATA_TABLE_ENTRY};
extern crate alloc;
#[derive(Debug, Default, Clone)]
pub struct ModuleInfo {
    pub base_address: usize,
    pub full_path: String,
    pub name: String,
}

pub unsafe fn enum_modules(
    driver: &mut DRIVER_OBJECT,
    call_back: &mut dyn FnMut(ModuleInfo) -> bool,
) {
    if driver.DriverSection.is_null() {
        return;
    }
    let module_list = driver.DriverSection.cast::<KLDR_DATA_TABLE_ENTRY>();
    let head = driver.DriverSection.cast::<LIST_ENTRY>();
    let mut entry = (*module_list).InLoadOrderLinks.Flink.cast_const();
    loop {
        if entry.is_null() || entry.eq(&head) {
            break;
        }
        let entity = entry.cast::<KLDR_DATA_TABLE_ENTRY>();

        let base = (*entity).DllBase as usize;
        if base > 0 {
            let base_name = unicode_string_to_string(&(*entity).BaseDllName);
            let full_path = unicode_string_to_string(&(*entity).FullDllName);

            if call_back(ModuleInfo {
                base_address: base,
                name: base_name,
                full_path: full_path,
            }) {
                break;
            }
        }
        entry = (*entry).Flink;
    }
}
