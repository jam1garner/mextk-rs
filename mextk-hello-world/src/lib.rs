#![feature(lang_items)]
#![no_std]

use mextk_sys::{GOBJ, OSReport};

#[no_mangle]
pub extern "C" fn OnLoad(_: *const GOBJ) {
    unsafe {
        OSReport("Hello World\0".as_ptr() as _);
    }
}

// ==================== Language Features ====================

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    unsafe {
        core::hint::unreachable_unchecked()
    }
}

#[lang = "eh_personality"] extern fn rust_eh_personality() {}
