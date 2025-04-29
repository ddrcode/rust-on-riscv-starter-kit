#![no_std]
#![no_main]

use core::panic::PanicInfo;

const UART0: *mut u8 = 0x1000_0000 as *mut u8;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let msg = b"Hello, RISC-V!\n";
    for &b in msg {
        unsafe { UART0.write_volatile(b); }
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let msg = b"panic\n";
    for &b in msg {
        unsafe { UART0.write_volatile(b); }
    }
    loop {}
}
