use core::slice;

#[repr(transparent)]
pub struct Block<'a> {
    inner: &'a [u8],
}

impl<'a> Block<'a> {
    pub fn from_ptr(ptr: *const u8, size: usize) -> Self {
        Self {
            inner: unsafe { slice::from_raw_parts(ptr, size) },
        }
    }
}

impl<'a> core::ops::Deref for Block<'a> {
    type Target = &'a [u8];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
