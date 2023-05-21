#![no_std]
#![no_main]

use uefi::prelude::*;
use uefi_services::{self, print};

use developing_modules::x86_64::port_io::{inb, outb};

#[cfg(not(target_arch = "x86_64"))]
compile_error!("Target needs to be x86_64");

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[entry]
fn _entry_stage1(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    const PORT: u16 = 0x3f8;

    outb(PORT + 1, 0x00);
    outb(PORT + 3, 0x80);
    outb(PORT + 0, 0x03);
    outb(PORT + 1, 0x00);
    outb(PORT + 3, 0x03);
    outb(PORT + 2, 0xc7);
    outb(PORT + 4, 0x0b);
    outb(PORT + 4, 0x1e);
    outb(PORT, 0xae);
    print!("{:#x}", inb(PORT));

    loop {}

    Status::SUCCESS
}
