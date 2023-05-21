#![no_std]
#![no_main]

use core::arch::asm;
use core::ffi::c_void;
use core::fmt::Write;

use developing_modules::{
    serial::Serial,
    x86_64::{gdt::Gdtr, uart::*},
};

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
        serial
            .serial_write_str("Serial port initialized.\n")
            .unwrap();
    }

    let mut gdtr: Gdtr = Gdtr { base: 0, limit: 0 };
    unsafe {
        asm!("sgdt [{}]", in(reg) &mut gdtr, options(nostack, preserves_flags));
    }
    writeln!(serial, "{:?}", gdtr).unwrap();

    let table = unsafe { gdtr.descriptor_table() };
    writeln!(serial, "{} Descriptors", table.len()).unwrap();
    for descriptor in table {
        writeln!(serial, "{:?}", descriptor).unwrap();
    }

    loop {}
}
