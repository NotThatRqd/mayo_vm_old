use std::collections::VecDeque;
use crate::devices::device::Device;

struct Region {
    device: Box<dyn Device>,

    start: usize,
    end: usize,

    remap: bool,
}

pub struct MemoryMapper {
    regions: VecDeque<Region>,
}

impl MemoryMapper {
    pub fn new() -> Self<> {
        Self {
            regions: VecDeque::new(),
        }
    }

    pub fn map(&mut self, device: Box<dyn Device>, start: usize, end: usize, remap: bool) {
        let region = Region {
            device,
            start,
            end,
            remap,
        };
        self.regions.push_front(region);

        // TODO: add way to remove region from regions list
    }

    fn find_region(&self, address: usize) -> Option<&Region> {
        self.regions.iter().find(|r| address >= r.start && address <= r.end)
    }

    fn find_mut_region(&mut self, address: usize) -> Option<&mut Region> {
        self.regions.iter_mut().find(|r| address >= r.start && address <= r.end)
    }
}

impl Device for MemoryMapper {
    fn read_at_u8(&self, offset: usize) -> Option<u8> {
        let region = self.find_region(offset)
            .unwrap();

        let final_address;
        if region.remap {
            final_address = offset - region.start;
        } else {
            final_address = offset;
        }

        region.device.read_at_u8(final_address)
    }

    fn read_at_u16(&self, offset: usize) -> Option<u16> {
        let region = self.find_region(offset)
            .unwrap();

        let final_address;
        if region.remap {
            final_address = offset - region.start;
        } else {
            final_address = offset;
        }

        region.device.read_at_u16(final_address)
    }

    fn write_at_u8(&mut self, offset: usize, num: u8) -> Result<(), ()> {
        let region = self.find_mut_region(offset)
            .unwrap();

        let final_address;
        if region.remap {
            final_address = offset - region.start;
        } else {
            final_address = offset;
        }

        region.device.write_at_u8(final_address, num)
    }

    fn write_at_u16(&mut self, offset: usize, num: u16) -> Result<(), ()> {
        let region = self.find_mut_region(offset)
            .unwrap();

        let final_address;
        if region.remap {
            final_address = offset - region.start;
        } else {
            final_address = offset;
        }

        region.device.write_at_u16(final_address, num)
    }
}
