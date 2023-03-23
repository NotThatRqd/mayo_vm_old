use crate::devices::device::Device;

pub struct ScreenDevice;

impl ScreenDevice {
    pub fn new() -> Self {
        Self
    }
}

impl Device for ScreenDevice {
    fn read_at_u8(&self, _offset: usize) -> Option<u8> {
        None
    }

    fn read_at_u16(&self, _offset: usize) -> Option<u16> {
        None
    }

    fn write_at_u8(&mut self, offset: usize, num: u8) -> Result<(), ()> {
        todo!()
    }

    fn write_at_u16(&mut self, offset: usize, num: u16) -> Result<(), ()> {
        todo!()
    }
}
