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
use winapi::km::wdm::IO_PRIORITY::IO_NO_INCREMENT;
use winapi::km::wdm::{
    IoCompleteRequest, IoCreateDevice, IoCreateSymbolicLink, IoDeleteDevice, IoDeleteSymbolicLink,
    DEVICE_OBJECT, DEVICE_TYPE, IRP, IRP_MJ, PDEVICE_OBJECT,
};
use winapi::shared::ntdef::{FALSE, NT_SUCCESS};
use winapi::{
    km::wdm::DRIVER_OBJECT,
    shared::{
        ntdef::{NTSTATUS, PVOID},
        ntstatus::STATUS_SUCCESS,
    },
};

use crate::control::dispatch_device_control;
use crate::lang::unicode_string;
use crate::module::enum_modules;
extern crate alloc;

pub mod control;
pub mod lang;
pub mod module;
pub mod nt;
const DEVICE_NAME: &'static str = "\\Device\\WindowsKernelResearch";
const SYMBOL_LINK: &'static str = "\\??\\WindowsKernelResearch";

pub extern "system" fn dispatch_default_routine(
    _device: &mut DEVICE_OBJECT,
    irp: &mut IRP,
) -> NTSTATUS {
    irp.IoStatus.Information = 0;
    unsafe {
        *(irp.IoStatus.__bindgen_anon_1.Status_mut()) = STATUS_SUCCESS;
        IoCompleteRequest(irp, IO_NO_INCREMENT);
    }
    log::info!("Call Dispatch Default Routine");
    STATUS_SUCCESS
}

pub extern "system" fn driver_unload(driver: &mut DRIVER_OBJECT) {
    log::info!("Call Driver Unload");

    // delete symbol link & device
    let symbol_link = unicode_string(obfstr::wide!(SYMBOL_LINK));
    unsafe {
        if !driver.DeviceObject.is_null() {
            IoDeleteDevice(driver.DeviceObject);
        }
        IoDeleteSymbolicLink(&symbol_link);
    }
}

#[no_mangle]
pub extern "system" fn driver_entry(driver: &mut DRIVER_OBJECT, _path: PVOID) -> NTSTATUS {
    KernelLogger::init(LevelFilter::Info).unwrap();
    log::info!("Call Driver Entry");

    driver.DriverUnload = Some(driver_unload);
    driver.MajorFunction[IRP_MJ::CREATE as usize] = Some(dispatch_default_routine);
    driver.MajorFunction[IRP_MJ::CLOSE as usize] = Some(dispatch_default_routine);
    driver.MajorFunction[IRP_MJ::READ as usize] = Some(dispatch_default_routine);
    driver.MajorFunction[IRP_MJ::WRITE as usize] = Some(dispatch_default_routine);
    driver.MajorFunction[IRP_MJ::DEVICE_CONTROL as usize] = Some(dispatch_device_control);

    // create device object & symbol link
    let device_name = unicode_string(obfstr::wide!(DEVICE_NAME));
    let symbol_link = unicode_string(obfstr::wide!(SYMBOL_LINK));
    let mut device_object: PDEVICE_OBJECT = core::ptr::null_mut();

    unsafe {
        let mut status = IoCreateDevice(
            driver,
            0,
            &device_name,
            DEVICE_TYPE::FILE_DEVICE_UNKNOWN,
            0,
            FALSE,
            &mut device_object,
        );
        if !NT_SUCCESS(status) {
            log::error!("failed to create device object {:#x}", status);
            return status;
        }
        status = IoCreateSymbolicLink(&symbol_link, &device_name);
        if !NT_SUCCESS(status) {
            log::error!("failed to create symbolic link {:#x}", status);
            return status;
        }
    }

    enum_modules(driver, &mut |module_info| -> bool {
        log::info!(
            "{} {:x} {}",
            module_info.name,
            module_info.base_address,
            module_info.full_path
        );
        false
    });

    STATUS_SUCCESS
}
