#![no_std]
#![no_main]

use core::ffi::c_void;

use developing_modules::{serial::Serial, x86_64::uart::*};

#[cfg(not(target_arch = "x86_64"))]
compile_error!("Target needs to be x86_64");

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[export_name = "efi_main"]
extern "efiapi" fn entry(_image_handle: *const c_void, mut _system_table: *const c_void) -> u64 {
    let mut serial = UartX86::new(
        UartX86Port::Com1,
        UartX86Baud::Baud38400,
        UartX86DataBits::Bits8,
        UartX86StopBits::Bits1,
        UartX86Parity::None,
    );

    if serial.init().is_ok() {
        for b in "hello world!".bytes() {
            serial.write_byte(b).unwrap();
        }
    }

    loop {}
}
