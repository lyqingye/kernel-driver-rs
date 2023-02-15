use crate::nt::KeBugCheck;
use alloc::string::String;
use core::panic::PanicInfo;
use kernel_alloc::KernelAlloc;
use winapi::shared::ntdef::UNICODE_STRING;
extern crate alloc;
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

#[inline]
pub fn unicode_string(s: &[u16]) -> UNICODE_STRING {
    let len = s.len();
    let n = if len > 0 && s[len - 1] == 0 {
        len - 1
    } else {
        len
    };

    UNICODE_STRING {
        Length: (n * 2) as u16,
        MaximumLength: (len * 2) as u16,
        Buffer: s.as_ptr() as _,
    }
}

#[inline]
pub unsafe fn unicode_string_to_string(unicode: &UNICODE_STRING) -> String {
    let slice = core::slice::from_raw_parts(unicode.Buffer, unicode.Length as usize / 2);
    String::from_utf16_lossy(slice)
}
