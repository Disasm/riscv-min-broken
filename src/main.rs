#![allow(warnings)]
#![no_std]
#![no_main]

//extern crate panic_halt;
use core::panic::PanicInfo;
#[panic_handler]
fn panic_handler(arg: &PanicInfo) -> ! {
    loop {}
}

//use riscv::register::mtvec;
//use riscv_rt::entry;

/*use core::sync::atomic::{AtomicUsize, Ordering};

#[no_mangle]
pub fn foo(x: &AtomicUsize, y: usize) -> usize {
    x.swap(y, Ordering::SeqCst)
}*/

//use core::f32;

extern "C" {
    fn consume(value: usize);
}

extern "C" {
    // Boundaries of the .bss section
    static mut _ebss: u32;
    static mut _sbss: u32;

    // Boundaries of the .data section
    static mut _edata: u32;
    static mut _sdata: u32;

    // Initial values of the .data section (stored in Flash)
    static _sidata: u32;
}

#[link_section = ".init.rust"]
#[export_name = "_start_rust"]
pub extern "C" fn start_rust() -> ! {
    unsafe {
        let ptr = consume as *const fn(usize) as usize;
        consume(ptr);
        main();
    }
}

#[inline(never)]
pub fn main() -> ! {
    unsafe { consume(42); }
    //let ptr = main as *const u32 as usize;
//    test_fp();
    //test_atomics();
    loop {}
}

pub extern "C" fn _start_trap_rust() -> ! {
    loop {}
}

#[link_section = ".trap.rust"]
#[export_name = "_start_trap_rust"]
pub extern "C" fn start_trap_rust() {
    loop {}
}

/*#[export_name = "abort"]
pub fn abort1() {
    loop {}
}*/
