#![no_std]
#![no_main]

use core::arch::global_asm;

#[cfg(not(target_arch = "riscv64"))]
compile_error!("This binary needs to be compiled for riscv64");

global_asm!(include_str!("entry.S"));

#[panic_handler]
fn handle_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[link_section = ".text.boot"]
pub unsafe extern "C" fn _entry_stage1() -> ! {
    const UART: *mut u8 = 0x1000_0000 as *mut u8;

    const HELLO: &'static str = "Hello RiscV!";
    for byte in HELLO.bytes() {
        core::ptr::write_volatile(UART, byte);
    }

    loop {}
}
