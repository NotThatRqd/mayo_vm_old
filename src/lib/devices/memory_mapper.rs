use std::collections::VecDeque;
use crate::device::Device;

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
    pub fn new() -> Self {
        Self {
            regions: VecDeque::new(),
        }
    }


    /// Maps a device to a section of memory
    ///
    /// # Arguments
    ///
    /// * `device`: The device to map
    /// * `start`: The start of where to map it to
    /// * `end`: The end of where to map the device to
    /// * `remap`: tl;dr: set this parameter to true. If true, when someone tries to read or write where this device is mapped
    /// the location in memory will be converted so that this device doesn't even know it's in a
    /// Memory Mapper. For example, if you had a screen device from 0x3000 to 0x30FF and then tried
    /// to read on this Memory Mapper 0x3001, with remap off you would get the whatever the screen
    /// device returns for 0x3001, but with remap on you would get whatever the screen device
    /// returns for 0x0001
    ///
    /// # Examples
    ///
    /// ```
    /// use mayo_lib::devices::memory::Memory;
    /// use mayo_lib::devices::memory_mapper::MemoryMapper;
    /// use mayo_lib::devices::screen_device::ScreenDevice;
    ///
    /// let mem = Box::new(Memory::from_num_of_bytes(256*256));
    /// let screen_device = Box::new(ScreenDevice::new());
    /// let mut mm = MemoryMapper::new();
    /// mm.map(mem, 0, 0xFFFF, true);
    /// mm.map(screen_device, 0x3000, 0x30FF, true);
    /// ```
    pub fn map(&mut self, device: Box<dyn Device>, start: usize, end: usize, remap: bool) {
        assert!(start < end);

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
