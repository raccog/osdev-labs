#![no_std]
#![no_main]

use core::{arch::asm, ffi::c_void, fmt::Write, mem, ptr, slice};

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
    // Init serial device
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

    // Get memory map size
    let boot_services = (*system_table).boot_services();
    let map_size = (*boot_services).memory_map_size();
    if let Err(err) = map_size {
        writeln!(serial, "Error while getting memory map size: {:#x}", err).unwrap();
    };
    let map_size = map_size.unwrap() + mem::size_of::<UefiMemoryDescriptor>() * 2;

    // Allocate buffer for memory map
    let mut buffer: *mut c_void = ptr::null_mut();
    let status = ((*boot_services).allocate_pool)(
        LOADER_DATA,
        map_size as u64,
        &mut buffer
    );

    // Retrieve memory map from UEFI
    if status != SUCCESS {
        writeln!(serial, "Failed to allocate for memory map: {:#x}", status).unwrap();
    } else {
        let mut buffer = slice::from_raw_parts_mut(buffer as *mut u8, map_size);
        let map = (*boot_services).retrieve_memory_map(&mut buffer);
        match map {
            Ok(map) => writeln!(serial, "Got map!").unwrap(),
            Err(err) => writeln!(serial, "Error: {:#x}", err).unwrap(),
        };
    }

    // TODO: Set the GDT
    let mut gdtr: Gdtr = Gdtr { base: 0, limit: 0 };
    unsafe {
        asm!("sgdt [{}]", in(reg) &mut gdtr, options(nostack, preserves_flags));
    }
    //writeln!(serial, "{:?}", gdtr).unwrap();

    let _table = unsafe { gdtr.descriptor_table() };
    //writeln!(serial, "{} Descriptors", table.len()).unwrap();
    //for descriptor in table {
    //    writeln!(serial, "{:?}", descriptor).unwrap();
    //}

    loop {}
}
