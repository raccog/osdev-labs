#![no_std]
#![no_main]

use core::arch::asm;
use core::ffi::c_void;
use core::fmt::Write;

use developing_modules::{
    firmware::uefi::memory_map::*,
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
unsafe extern "efiapi" fn entry(_image_handle: *const c_void, system_table: *const UefiSystemTable) -> u64 {
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

    let boot_services = (*system_table).boot_services();
    let map_size = (*boot_services).memory_map_size();
    match map_size {
        Ok(map_size) => writeln!(serial, "{}", map_size).unwrap(),
        Err(err) => writeln!(serial, "Error: {:#x}", err).unwrap(),
    };

    // TODO: Allocate buffer
    let map = (*boot_services).retrieve_memory_map();
    match map {
        Ok(map) => writeln!(serial, "Got map!").unwrap(),
        Err(err) => writeln!(serial, "Error: {:#x}", err).unwrap(),
    };

    let mut gdtr: Gdtr = Gdtr { base: 0, limit: 0 };
    unsafe {
        asm!("sgdt [{}]", in(reg) &mut gdtr, options(nostack, preserves_flags));
    }
    writeln!(serial, "{:?}", gdtr).unwrap();

    let table = unsafe { gdtr.descriptor_table() };
    //writeln!(serial, "{} Descriptors", table.len()).unwrap();
    //for descriptor in table {
    //    writeln!(serial, "{:?}", descriptor).unwrap();
    //}

    loop {}
}
