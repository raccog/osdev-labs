#![no_std]
#![no_main]

use core::ffi::c_void;

use developing_modules::x86_64::port_io::{inb, outb};

#[cfg(not(target_arch = "x86_64"))]
compile_error!("Target needs to be x86_64");

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[export_name = "efi_main"]
extern "efiapi" fn entry(_image_handle: *const c_void, mut _system_table: *const c_void) -> u64 {
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
    if inb(PORT) == 0xae {
        outb(PORT + 4, 0x0f);
        outb(PORT, b'!');
        loop {}
    }

    0
}
