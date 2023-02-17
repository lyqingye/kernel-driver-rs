use winapi::{
    km::wdm::{
        IoCompleteRequest, IoGetCurrentIrpStackLocation, DEVICE_OBJECT,
        IO_PRIORITY::IO_NO_INCREMENT, IRP,
    },
    shared::{ntdef::NTSTATUS, ntstatus::STATUS_SUCCESS},
};

pub extern "system" fn dispatch_device_control(
    _device_object: &mut DEVICE_OBJECT,
    irp: &mut IRP,
) -> NTSTATUS {
    let stack = IoGetCurrentIrpStackLocation(irp);
    let input_buffer_length;
    let output_buffer_length;
    let input_buffer;
    let output_buffer;
    unsafe {
        input_buffer_length = (*stack).Parameters.DeviceIoControl().InputBufferLength as usize;
        output_buffer_length = (*stack).Parameters.DeviceIoControl().OutputBufferLength as usize;
        input_buffer = core::slice::from_raw_parts(
            irp.AssociatedIrp.SystemBuffer_mut().cast() as *const u8,
            input_buffer_length,
        );
        output_buffer = core::slice::from_raw_parts_mut(
            irp.AssociatedIrp.SystemBuffer_mut().cast() as *mut u8,
            output_buffer_length,
        );
    }

    let code = unsafe { (*stack).Parameters.DeviceIoControl().IoControlCode };
    let status = STATUS_SUCCESS;
    let information: usize = 0;

    log::info!("Code: {:x}", code);
    log::info!(
        "Input_buffer: {:x} length: {}",
        input_buffer.as_ptr() as usize,
        input_buffer_length
    );
    log::info!(
        "Output_buffer: {:x} length: {}",
        output_buffer.as_ptr() as usize,
        output_buffer_length
    );

    // complete request
    irp.IoStatus.Information = information;
    unsafe {
        *(irp.IoStatus.__bindgen_anon_1.Status_mut()) = status;
        IoCompleteRequest(irp, IO_NO_INCREMENT);
    }
    return status;
}
