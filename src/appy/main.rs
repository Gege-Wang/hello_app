#![no_std]
#![no_main]


#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    core::arch::asm!("
        wfi",
        options(noreturn),
        )
}
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
