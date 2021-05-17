#![no_std]

use core::panic::PanicInfo;

extern "C" {
    pub fn abort() -> !;
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { abort() }
}
