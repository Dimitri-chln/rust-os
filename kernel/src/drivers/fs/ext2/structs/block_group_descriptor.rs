#[repr(C, packed)]
pub struct BlockGroupDescriptor {
    /// Block address of block usage bitmap
    ///
    /// - Bytes 0-3
    pub block_usage_bitmap_block_number: u32,
    /// Block address of inode usage bitmap
    ///
    /// - Bytes 4-7
    pub inode_usage_bitmap_block_number: u32,
    /// Starting block address of inode table
    ///
    /// - Byte 8-11
    pub inode_table_starting_block_number: u32,
    /// Number of unallocated blocks in group
    ///
    /// - Bytes 12-13
    pub unallocated_blocks: u16,
    /// Number of unallocated inodes in group
    ///
    /// - Bytes 14-15
    pub unallocated_inodes: u16,
    /// Number of directories in group
    ///
    /// - Bytes 16-17
    pub total_directories: u16,
    /// Unused
    ///
    /// - Bytes 18-31
    _unused_1: [u8; 14],
}

impl BlockGroupDescriptor {
    pub const SIZE: usize = 32;
}
