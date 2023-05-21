use core::fmt::Write;

use crate::{
    serial::{Error as SerialError, Serial},
    x86_64::port_io::{inb, outb},
};

#[repr(u16)]
#[derive(Copy, Clone, Debug)]
pub enum UartX86Port {
    Com1 = 0x3f8,
}

#[repr(u16)]
#[derive(Copy, Clone, Debug)]
pub enum UartX86Baud {
    Baud115200 = 1,
    Baud57600 = 2,
    Baud38400 = 3,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum UartX86DataBits {
    Bits5 = 0,
    Bits6 = 1,
    Bits7 = 2,
    Bits8 = 3,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum UartX86StopBits {
    Bits1 = 0,
    Bits2 = 1,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum UartX86Parity {
    None = 0,
    Odd = 1,
    Even = 2,
    Mark = 3,
    Space = 4,
}

#[derive(Copy, Clone, Debug)]
pub struct UartX86 {
    port: UartX86Port,
    baud: UartX86Baud,
    data_bits: UartX86DataBits,
    stop_bits: UartX86StopBits,
    parity: UartX86Parity,
    is_initialized: bool,
}

impl UartX86 {
    pub fn new(
        port: UartX86Port,
        baud: UartX86Baud,
        data_bits: UartX86DataBits,
        stop_bits: UartX86StopBits,
        parity: UartX86Parity,
    ) -> Self {
        Self {
            port,
            baud,
            data_bits,
            stop_bits,
            parity,
            is_initialized: false,
        }
    }
}

impl Serial for UartX86 {
    fn init(&mut self) -> Result<(), SerialError> {
        if self.is_initialized {
            return Ok(());
        }

        let port = self.port as u16;

        // Disable interrupts
        outb(port + 1, 0x00);
        // Enable DLAB to set baud rate divisor
        outb(port + 3, 0x80);
        // Set baud rate divisor
        let [baud_lo, baud_hi] = (self.baud as u16).to_ne_bytes();
        outb(port + 0, baud_lo);
        outb(port + 1, baud_hi);
        // Set data bits, stop bits, and parity
        let mode: u8 =
            (self.data_bits as u8) | ((self.stop_bits as u8) << 2) | ((self.parity as u8) << 3);
        outb(port + 3, mode);
        // Enable and clear FIFOs and set their interrupt trigger level to 14 bytes
        outb(port + 2, 0xc7);
        // Enable interrupt requests, enable Request To Send (RTS) and Data Terminal Ready (DTR) pins
        outb(port + 4, 0x0b);
        // Set in loopback mode to test serial hardware
        outb(port + 4, 0x1e);

        // Send test value
        const TEST_VALUE: u8 = 0xae;
        outb(port, TEST_VALUE);

        // Return error if test value was not looped back
        if inb(port) == TEST_VALUE {
            // Disable loopback, enable interrupts, enable OUT1 and OUT2 pins
            outb(port + 4, 0x0f);
            self.is_initialized = true;
        } else {
            return Err("Serial port failed to initialize");
        }

        Ok(())
    }

    fn read_byte(&mut self) -> Result<u8, SerialError> {
        if !self.is_initialized {
            return Err("Tried to read a byte using uninitialized serial port");
        }

        let value = inb(self.port as u16);

        Ok(value)
    }

    fn write_byte(&mut self, value: u8) -> Result<(), SerialError> {
        if !self.is_initialized {
            return Err("Tried to send a byte using uninitialized serial port");
        }

        outb(self.port as u16, value);

        Ok(())
    }
}

impl Write for UartX86 {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if !self.is_initialized {
            return Err(core::fmt::Error);
        }

        for b in s.bytes() {
            self.write_byte(b).unwrap();
        }

        Ok(())
    }
}
