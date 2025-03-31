use core::slice;

use super::inode::Inode;

pub struct InodeTable<'a> {
    inner: &'a [Inode],
}

impl<'a> InodeTable<'a> {
    pub fn from_ptr(ptr: *const Inode, size: usize) -> Self {
        // FIXME: inode size may not be 128
        Self {
            inner: unsafe { slice::from_raw_parts(ptr, size) },
        }
    }
}

impl<'a> core::ops::Deref for InodeTable<'a> {
    type Target = &'a [Inode];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
