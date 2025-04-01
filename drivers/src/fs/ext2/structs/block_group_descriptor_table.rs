use core::slice;

use super::block_group_descriptor::BlockGroupDescriptor;

pub struct BlockGroupDescriptorTable<'a> {
    inner: &'a [BlockGroupDescriptor],
}

impl<'a> BlockGroupDescriptorTable<'a> {
    pub fn from_ptr(ptr: *const BlockGroupDescriptor, size: usize) -> Self {
        Self {
            inner: unsafe { slice::from_raw_parts(ptr, size) },
        }
    }
}

impl<'a> core::ops::Deref for BlockGroupDescriptorTable<'a> {
    type Target = &'a [BlockGroupDescriptor];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
