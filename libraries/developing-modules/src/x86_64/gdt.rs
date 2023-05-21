use core::{mem, slice};

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct SegmentDescriptor {
    limit_lo: u16,
    base0: u16,
    base1: u8,
    fields0: u8,
    fields1: u8,
    base2: u8,
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct Gdtr {
    pub limit: u16,
    pub base: u64,
}

impl SegmentDescriptor {
    pub fn base(&self) -> u32 {
        self.base0() | (self.base1() << 16) | (self.base2() << 24)
    }

    pub fn granularity(&self) -> u8 {
        (self.fields1 & 0x80) >> 7
    }

    pub fn is_64bits(&self) -> bool {
        (self.fields1 & 0x20) == 0x20
    }

    pub fn is_available(&self) -> bool {
        (self.fields1 & 0x10) == 0x10
    }

    pub fn is_present(&self) -> bool {
        (self.fields0 & 0x80) == 0x80
    }

    pub fn is_system_segment(&self) -> bool {
        (self.fields0 & 0x10) == 0x10
    }

    pub fn limit(&self) -> u32 {
        self.limit_lo() | (self.limit_hi() << 16)
    }

    pub fn operation_size(&self) -> u8 {
        (self.fields1 & 0x40) >> 6
    }

    pub fn privilege_level(&self) -> u8 {
        (self.fields0 & 0x60) >> 5
    }

    pub fn segment_type(&self) -> u8 {
        self.fields0 & 0xf
    }

    fn base0(&self) -> u32 {
        self.base0 as u32
    }

    fn base1(&self) -> u32 {
        self.base1 as u32
    }

    fn base2(&self) -> u32 {
        self.base2 as u32
    }

    fn limit_hi(&self) -> u32 {
        (self.fields1 & 0x0f) as u32
    }

    fn limit_lo(&self) -> u32 {
        self.limit_lo as u32
    }
}

impl Gdtr {
    pub unsafe fn descriptor_table(&self) -> &[SegmentDescriptor] {
        let length = (self.limit + 1) as usize / mem::size_of::<SegmentDescriptor>();
        slice::from_raw_parts(self.base as *const u8 as *const SegmentDescriptor, length)
    }
}

impl core::fmt::Debug for SegmentDescriptor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let base = self.base();
        let limit = self.limit();
        let segment_type = self.segment_type();
        let system_segment = self.is_system_segment();
        let privilege_level = self.privilege_level();
        let present = self.is_present();
        let operation_size = self.operation_size();
        let granularity = self.granularity();
        let is_64bits = self.is_64bits();
        let is_available = self.is_available();

        writeln!(f, "SegmentDescriptor {{")?;
        writeln!(f, "   base: {:#x}", base)?;
        writeln!(f, "   limit: {:#x}", limit)?;
        writeln!(f, "   type: {:#x}", segment_type)?;
        writeln!(f, "   system segment: {}", system_segment)?;
        writeln!(f, "   privilege level: {:#x}", privilege_level)?;
        writeln!(f, "   present: {}", present)?;
        writeln!(f, "   operation size: {:#x}", operation_size)?;
        writeln!(f, "   granularity: {}", granularity)?;
        writeln!(f, "   64 bits: {}", is_64bits)?;
        writeln!(f, "   available: {}", is_available)?;
        writeln!(f, "}}")?;

        Ok(())
    }
}

impl core::fmt::Debug for Gdtr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let base = self.base;
        let limit = self.limit;
        writeln!(f, "GDTR {{")?;
        writeln!(f, "   base: {:#x}", base)?;
        writeln!(f, "   limit: {:#x}", limit)?;
        writeln!(f, "}}")?;

        Ok(())
    }
}
