use core::slice;

pub trait Disk: Inner {
    fn block_size(&self) -> usize {
        Inner::block_size(self)
    }

    fn read(&mut self, block: u32) -> &[u8] {
        self.set_block_number(block);
        self.block_data()
    }

    fn write(&mut self, block: u32, data: &[u8]) {
        self.set_block_number(block);
        self.set_block_data(data);
    }
}

pub trait Inner {
    // Block size
    fn block_size(&self) -> usize;
    // Raw pointers
    fn block_number_ptr(&self) -> *const u32;
    fn block_data_ptr(&self) -> *const u8;

    /**
     * Auto-implemented functions
     */

    fn block_number(&self) -> u32 {
        unsafe { *self.block_number_ptr() }
    }

    fn set_block_number(&mut self, block: u32) {
        unsafe {
            self.block_number_ptr().cast_mut().write(block);
        }
    }

    fn block_data(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.block_data_ptr(), self.block_size()) }
    }

    fn set_block_data(&mut self, data: &[u8]) {
        unsafe { slice::from_raw_parts_mut(self.block_data_ptr().cast_mut(), self.block_size()) }
            .copy_from_slice(data);
    }
}

impl<T: Inner> Disk for T {}
