use super::block_group_descriptor::BlockGroupDescriptor;

#[repr(transparent)]
pub struct BlockGroupDescriptorTable {
    inner: &'static [BlockGroupDescriptor],
}

impl core::ops::Deref for BlockGroupDescriptorTable {
    type Target = &'static [BlockGroupDescriptor];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BlockGroupDescriptorTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
