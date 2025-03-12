use x86_64::VirtAddr;

use super::structs::{inode::Inode, superblock::SuperBlock};

pub struct System {
    pub(super) superblock: &'static SuperBlock,
}

impl System {
    /// Safety: `volume_start` must be the start address of the volume
    pub unsafe fn new(volume_start: VirtAddr) -> Option<Self> {
        Some(Self {
            superblock: SuperBlock::new(volume_start)?,
        })
    }

    pub fn root_inode(&self) -> &Inode {
        self.superblock.inode(2)
    }
}
