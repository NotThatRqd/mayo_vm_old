use data_view::View;
use mayo_lib::device::Device;

/// The most basic implementation of the Device trait that simply uses a Vec<u8> under the hood.
pub struct Memory {
    internal_mem: Vec<u8>
}

impl Memory {
    pub fn from_num_of_bytes(bytes: usize) -> Self {
        Self {
            internal_mem: vec![0; bytes],
        }
    }

    pub fn from_vec(mem: Vec<u8>) -> Self {
        Self {
            internal_mem: mem,
        }
    }
}

impl Device for Memory {
    fn read_at_u8(&self, offset: usize) -> Option<u8> {
        self.internal_mem.read_at::<u8>(offset)
    }

    fn read_at_u16(&self, offset: usize) -> Option<u16> {
        self.internal_mem.read_at::<u16>(offset)
    }

    fn write_at_u8(&mut self, offset: usize, num: u8) -> Result<(), ()> {
        self.internal_mem.write_at::<u8>(offset, num)
    }

    fn write_at_u16(&mut self, offset: usize, num: u16) -> Result<(), ()> {
        self.internal_mem.write_at::<u16>(offset, num)
    }
}