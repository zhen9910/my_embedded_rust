#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::arch::asm;
use core::panic::PanicInfo;

mod boot {
    use core::arch::global_asm;

    global_asm!(".section .text._start");
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    unsafe {
        // turn PIN21 into an output
        core::ptr::write_volatile(0x3F20_0008 as *mut u32, 1 << 3);
        loop {
            // turn PIN21 on
            core::ptr::write_volatile(0x3F20_001C as *mut u32, 1 << 21);
            for _ in 0..100_000 {
                asm!("nop");
            }
            // turn PIN21 off
            core::ptr::write_volatile(0x3F20_0028 as *mut u32, 1 << 21);
            for _ in 0..100_000 {
                asm!("nop");
            }
        }
    }

    // unsafe {
    //     let gpio = 0x3F20_0008 as *mut u32;
    //     *gpio.offset(1) = 1 << 3;
    //     loop {
    //         // turn PIN21 on
    //         *gpio.offset(7) = 1 << 21;
    //         for _ in 0..100_000 {}
    //         // turn PIN21 off
    //         *gpio.offset(10) = 1 << 21;
    //         for _ in 0..100_000 {
    //             asm!("nop");
    //         }
    //     }
    // }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
