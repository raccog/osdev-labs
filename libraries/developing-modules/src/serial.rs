pub type Error = &'static str;

pub trait Serial {
    fn init(&mut self) -> Result<(), Error>;

    fn is_initialized(&self) -> bool;

    fn read_byte(&mut self) -> Result<u8, Error>;

    fn serial_write_str(&mut self, value: &str) -> Result<(), Error> {
        if !self.is_initialized() {
            return Err("Tried to write a string to an uninitialized serial");
        }

        for byte in value.bytes() {
            self.write_byte(byte).unwrap();
        }

        Ok(())
    }

    fn write_byte(&mut self, value: u8) -> Result<(), Error>;
}
