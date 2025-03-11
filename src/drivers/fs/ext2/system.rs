use x86_64::VirtAddr;

use super::structs::SuperBlock;

pub struct System {
    superblock: &'static SuperBlock,
}

impl System {
    /// Safety: `volume_start` must be the start address of the volume
    pub unsafe fn new(volume_start: VirtAddr) -> Option<Self> {
        Some(Self {
            superblock: SuperBlock::new(volume_start)?,
        })
    }
}
