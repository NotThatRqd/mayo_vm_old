use std::io::Write;
use console::Term;
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

    fn write_at_u8(&mut self, _offset: usize, _num: u8) -> Result<(), ()> {
        Err(())
    }

    fn write_at_u16(&mut self, offset: usize, num: u16) -> Result<(), ()> {
        let x = offset % 16;
        let y = (offset as f64 / 16.0).floor() as usize;

        // multiplied by 2 because it looks better
        Term::stdout().move_cursor_to(x * 2, y)
            .unwrap();

        let character_value = (num & 0x00FF) as u8;

        print!("{}", character_value as char);

        Ok(())
    }
}
