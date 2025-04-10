use super::block::Block;
use super::block_group_descriptor::BlockGroupDescriptor;
use super::inode::Inode;
use super::inode_table::InodeTable;
use super::superblock::SuperBlock;

pub struct BlockGroup<'a> {
    pub descriptor: &'a BlockGroupDescriptor,
}

impl BlockGroup<'_> {
    pub fn block_usage_bitmap<'a>(&self, superblock: &'a SuperBlock) -> Block<'a> {
        let block_usage_bitmap_ptr = superblock
            .block_ptr(self.descriptor.block_usage_bitmap_block_number)
            .expect("block_usage_bitmap out of range");

        Block::from_ptr(block_usage_bitmap_ptr, superblock.block_size() as usize)
    }

    pub fn inode_usage_bitmap<'a>(&self, superblock: &'a SuperBlock) -> Block<'a> {
        let inode_usage_bitmap_ptr = superblock
            .block_ptr(self.descriptor.inode_usage_bitmap_block_number)
            .expect("block_usage_bitmap out of range");

        Block::from_ptr(inode_usage_bitmap_ptr, superblock.block_size() as usize)
    }

    pub fn inode_table<'a>(&self, superblock: &'a SuperBlock) -> InodeTable<'a> {
        let inode_table_start_ptr = superblock
            .block_ptr(self.descriptor.inode_table_starting_block_number)
            .expect("inode table out of range") as *const Inode;

        InodeTable::from_ptr(inode_table_start_ptr, superblock.inodes_per_group as usize)
    }
}
