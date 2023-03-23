use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use data_view::{Endian, View};

struct Region<T, U>
where
    T: Deref<Target = U> + DerefMut<>,
    U: View + ?Sized,
{
    device: T,

    start: usize,
    end: usize,

    remap: bool,
}

struct MemoryMapper<T, U>
where
    T: Deref<Target = U> + DerefMut<>,
    U: View + ?Sized,
{
    regions: VecDeque<Region<T, U>>,
}

impl<T, U> MemoryMapper<T, U>
where
    T: Deref<Target = U> + DerefMut<>,
    U: View + ?Sized,
{
    pub fn new() -> Self<> {
        Self {
            regions: VecDeque::new(),
        }
    }

    pub fn map(&mut self, device: T, start: usize, end: usize, remap: bool) {
        let region = Region {
            device,
            start,
            end,
            remap,
        };
        self.regions.push_front(region);

        // TODO: add way to remove region from regions list
    }

    fn find_region(&self, address: usize) -> Option<&Region<T, U>> {
        self.regions.iter().find(|r| address >= r.start && address <= r.end)
    }

    fn find_mut_region(&mut self, address: usize) -> Option<&mut Region<T, U>> {
        self.regions.iter_mut().find(|r| address >= r.start && address <= r.end)
    }
}

impl<T, U> View for MemoryMapper<T, U>
where
    T: Deref<Target = U> + DerefMut<>,
    U: View + ?Sized,
{
    fn read_at<E: Endian>(&self, offset: usize) -> Option<E> {
        let region = self.find_region(offset)
            .unwrap();

        let final_address;
        if region.remap {
            final_address = offset - region.start;
        } else {
            final_address = offset;
        }

        region.device.read_at::<E>(final_address)
    }

    fn write_at<E: Endian>(&mut self, offset: usize, num: E) -> Result<(), ()> {
        let region = self.find_mut_region(offset)
            .unwrap();

        let final_address;
        if region.remap {
            final_address = offset - region.start;
        } else {
            final_address = offset;
        }

        region.device.write_at::<E>(final_address, num)
    }
}
