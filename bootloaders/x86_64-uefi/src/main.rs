#![no_std]
#![no_main]

use uefi::prelude::*;
use uefi_services;

#[cfg(not(target_arch = "x86_64"))]
compile_error!("Target needs to be x86_64");

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[entry]
fn _entry_stage1(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    Status::SUCCESS
}
