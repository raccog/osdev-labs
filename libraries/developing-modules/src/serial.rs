pub trait Serial {
    type Error = &'static str;

    fn init(&mut self) -> Result<(), Self::Error>;

    fn read_byte(&mut self) -> Result<u8, Self::Error>;

    fn write_byte(&mut self, value: u8) -> Result<(), Self::Error>;
}
