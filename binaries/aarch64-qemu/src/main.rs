#![no_std]
#![no_main]

#[cfg(not(target_arch = "aarch64"))]
compile_error!("This binary needs to be compiled for aarch64.");

use core::{arch::global_asm, fmt::Write};

use developing_modules::aarch64::pl011_uart::{Pl011Uart, UART0_BASE_ADDRESS};

// Include the start procedure
global_asm!(include_str!("entry_stage0.S"));

#[panic_handler]
fn handle_panic(info: &core::panic::PanicInfo) -> ! {
    let uart = unsafe { Pl011Uart::new(UART0_BASE_ADDRESS) };
    writeln!(uart, "[PANIC] {}", info).unwrap();
    loop {}
}

#[no_mangle]
#[link_section = ".text.boot"]
pub unsafe extern "C" fn _entry_stage1() -> ! {
    let uart = Pl011Uart::new(UART0_BASE_ADDRESS);

    writeln!(uart, "Hello UART!").unwrap();

    loop {}
}
