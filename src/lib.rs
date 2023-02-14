#![no_std]
#![feature(lang_items)]
#![feature(decl_macro)]
#![feature(box_syntax)]
#![feature(new_uninit)]
#![feature(link_llvm_intrinsics)]
#![allow(clippy::new_ret_no_self)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use kernel_log::KernelLogger;
use log::LevelFilter;
use winapi::{
    km::wdm::DRIVER_OBJECT,
    shared::{
        ntdef::{NTSTATUS, PVOID},
        ntstatus::STATUS_SUCCESS,
    },
};

pub mod lang;
pub mod nt;

pub extern "system" fn driver_unload(_driver: &mut DRIVER_OBJECT) {
    log::info!("Call Driver Unload");
}

#[no_mangle]
pub extern "system" fn driver_entry(driver: *mut DRIVER_OBJECT, _path: PVOID) -> NTSTATUS {
    KernelLogger::init(LevelFilter::Info).unwrap();
    log::info!("Call Driver Entry");
    unsafe { (*driver).DriverUnload = Some(driver_unload) };
    STATUS_SUCCESS
}
