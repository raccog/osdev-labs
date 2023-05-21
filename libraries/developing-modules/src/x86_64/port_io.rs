use core::arch::asm;

pub fn inb(port_addr: u16) -> u8 {
    let mut byte: u8;

    unsafe {
        asm!(
            "in al, dx",
            out("al") byte,
            in("dx") port_addr,
        );
    }

    byte
}

pub fn outb(port_addr: u16, value: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("al") value,
            in("dx") port_addr,
        );
    }
}
