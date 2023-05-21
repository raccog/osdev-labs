pub type Error = &'static str;

pub trait Serial {
    fn init(&mut self) -> Result<(), Error>;

    fn read_byte(&mut self) -> Result<u8, Error>;

    fn write_byte(&mut self, value: u8) -> Result<(), Error>;
}
