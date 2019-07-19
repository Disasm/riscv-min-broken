#![allow(warnings)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
#[panic_handler]
fn panic_handler(arg: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    fn consume(value: usize);
}

static mut BYTE: u8 = 1;

#[link_section = ".init.rust"]
#[export_name = "_start_rust"]
pub extern "C" fn start_rust() -> ! {
    unsafe {
        let ptr = consume as *const fn(usize) as usize;
        consume(ptr);
        consume(BYTE as usize);
    }

    loop {}
}

#[link_section = ".trap.rust"]
#[export_name = "_start_trap_rust"]
pub extern "C" fn start_trap_rust() {
    loop {}
}
