use alloc::string::String;
use winapi::{km::wdm::DRIVER_OBJECT, shared::ntdef::LIST_ENTRY};

use crate::{
    lang::read_unicode_string,
    nt::{MmIsAddressValid, KLDR_DATA_TABLE_ENTRY},
};
#[derive(Debug, Default, Clone)]
pub struct ModuleInfo {
    pub base_address: usize,
    pub full_path: String,
    pub name: String,
    pub size_of_image: usize,
}

pub fn get_ntos_module(driver: &mut DRIVER_OBJECT) -> Option<ModuleInfo> {
    let mut result = None;
    let ptr = MmIsAddressValid as usize;
    enum_modules(driver, &mut |info| -> bool {
        if ptr > info.base_address && ptr < (info.base_address + info.size_of_image) {
            result = Some(info);
            true
        } else {
            false
        }
    });
    result
}

pub fn enum_modules(driver: &mut DRIVER_OBJECT, call_back: &mut dyn FnMut(ModuleInfo) -> bool) {
    if driver.DriverSection.is_null() {
        return;
    }
    let module_list = driver.DriverSection.cast::<KLDR_DATA_TABLE_ENTRY>();
    let head = driver.DriverSection.cast::<LIST_ENTRY>();
    unsafe {
        let mut entry = (*module_list).InLoadOrderLinks.Flink.cast_const();
        loop {
            if entry.is_null() || entry.eq(&head) {
                break;
            }

            if !MmIsAddressValid(entry as _) {
                break;
            }

            let entity = entry.cast::<KLDR_DATA_TABLE_ENTRY>();

            let base = (*entity).DllBase as usize;
            if base > 0 {
                let base_name = read_unicode_string(&(*entity).BaseDllName, 512);
                let full_path = read_unicode_string(&(*entity).FullDllName, 512);

                if call_back(ModuleInfo {
                    base_address: base,
                    full_path: full_path,
                    name: base_name,
                    size_of_image: (*entity).SizeOfImage,
                }) {
                    break;
                }
            }
            entry = (*entry).Flink;
        }
    }
}
