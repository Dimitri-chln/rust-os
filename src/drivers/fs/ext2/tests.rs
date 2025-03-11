use crate::drivers::fs::ext2::structs::{BlockGroupDescriptor, SuperBlock};

#[test_case]
fn test_superblock_size() {
    assert_eq!(size_of::<SuperBlock>(), SuperBlock::SIZE);
}

#[test_case]
fn test_block_group_descriptor_size() {
    assert_eq!(
        size_of::<BlockGroupDescriptor>(),
        BlockGroupDescriptor::SIZE
    );
}
