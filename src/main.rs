#![no_std]
#![feature(panic_implementation)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
#![feature(try_from, nll)]

// TODO: Figure out how to get `failure` to drop its std dependency in `#[no_std]`
//#[macro_use]
//extern crate failure;
//#[cfg(test)]
//#[macro_use]
//extern crate hesl;
#[cfg(test)]
extern crate std;
mod consts;
mod vga;
use core::panic::PanicInfo;
use vga::Color;

const VGA_BUF_ADDR: usize = 0xb8000;

#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_something();
    loop {}
}

pub fn print_something() {
    let mut writer = vga::Writer::new(Color::Yellow, Color::Black, unsafe { &mut *(VGA_BUF_ADDR as *mut vga::Buffer) });

    writer.write_byte(b'H');
    writer.write_str("ello ");
    writer.write_str("Wörld!");
}
