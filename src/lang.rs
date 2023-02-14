use core::panic::PanicInfo;
use kernel_alloc::KernelAlloc;
use crate::nt::KeBugCheck;

#[no_mangle]
#[allow(bad_style)]
static _fltused: i32 = 0;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[no_mangle]
extern "C" fn __CxxFrameHandler3() {}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    log::info!("Panic handler called: {:?}", _info);
    unsafe { KeBugCheck(0x000000E2) };
}

#[global_allocator]
static GLOBAL: KernelAlloc = KernelAlloc;