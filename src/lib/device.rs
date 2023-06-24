use data_view::View;

/// Basically just a not-generic wrapper for `View`.
/// The reason this trait exists is so we can do `Box<dyn Device>` because `Box<dyn data_view::View>`
/// does not work because it is generic.
pub trait Device {
    fn read_at_u8(&self, offset: usize) -> Option<u8>;
    fn read_at_u16(&self, offset: usize) -> Option<u16>;

    fn write_at_u8(&mut self, offset: usize, num: u8) -> Result<(), ()>;
    fn write_at_u16(&mut self, offset: usize, num: u16) -> Result<(), ()>;
}

impl Device for Vec<u8> {
    fn read_at_u8(&self, offset: usize) -> Option<u8> {
        self.read_at::<u8>(offset)
    }

    fn read_at_u16(&self, offset: usize) -> Option<u16> {
        self.read_at::<u16>(offset)
    }

    fn write_at_u8(&mut self, offset: usize, num: u8) -> Result<(), ()> {
        self.write_at::<u8>(offset, num)
    }

    fn write_at_u16(&mut self, offset: usize, num: u16) -> Result<(), ()> {
        self.write_at::<u16>(offset, num)
    }
}