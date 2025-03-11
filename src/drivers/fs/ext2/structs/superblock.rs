use bitflags::bitflags;
use x86_64::VirtAddr;

use super::{
    block_group_descriptor::BlockGroupDescriptor,
    block_group_descriptor_table::BlockGroupDescriptorTable, inode::Inode,
};

#[derive(Debug)]
#[repr(C, packed)]
pub struct SuperBlock {
    /// Total number of inodes in file system
    ///
    /// - Bytes 0-3
    pub total_inodes: u32,
    /// Total number of blocks in file system
    ///
    /// - Bytes 4-7
    pub total_blocks: u32,
    /// Number of blocks reserved for superuser (see [`SuperBlock::user_reserved`])
    ///
    /// - Bytes 8-11
    pub reserved_blocks: u32,
    /// Total number of unallocated blocks
    ///
    /// - Bytes 12-15
    pub unallocated_blocks: u32,
    /// Total number of unallocated inodes
    ///
    /// - Bytes 16-19
    pub unallocated_inodes: u32,
    /// Block number of the block containing the superblock (also the starting block number, NOT always zero.)
    ///
    /// - Bytes 20-23
    pub block_number: u32,
    /// `log2(block size) - 10`. (In other words, the number to shift 1,024 to the left by to obtain the block size)
    ///
    /// - Bytes 24-27
    pub block_size_shift: u32,
    /// `log2(fragment size) - 10`. (In other words, the number to shift 1,024 to the left by to obtain the fragment size)
    ///
    /// - Bytes 28-31
    pub fragment_size_shift: u32,
    /// Number of blocks in each block group
    ///
    /// - Bytes 32-35
    pub blocks_per_group: u32,
    /// Number of fragments in each block group
    ///
    /// - Bytes 36-39
    pub fragments_per_group: u32,
    /// Number of inodes in each block group
    ///
    /// - Bytes 40-43
    pub inodes_per_group: u32,
    /// Last mount time (in [POSIX time](https://en.wikipedia.org/wiki/Unix_time))
    ///
    /// - Bytes 44-47
    pub last_mount_time: u32,
    /// Last written time (in [POSIX time](https://en.wikipedia.org/wiki/Unix_time))
    ///
    /// - Bytes 48-51
    pub last_written_time: u32,
    /// Number of times the volume has been mounted since its last consistency check ([fsck](https://en.wikipedia.org/wiki/Fsck))
    ///
    /// - Bytes 52-53
    pub mounts_since_consistency_check: u16,
    /// Number of mounts allowed before a consistency check ([fsck](https://en.wikipedia.org/wiki/Fsck)) must be done
    ///
    /// - Bytes 54-55
    pub mounts_before_consistency_check: u16,
    /// Ext2 signature (0xef53), used to help confirm the presence of Ext2 on a volume
    ///
    /// - Bytes 56-57
    pub signature: Signature,
    /// File system state
    ///
    /// - Bytes 58-59
    pub state: State,
    /// What to do when an error is detected
    ///
    /// - Bytes 60-61
    pub error_handling: ErrorHandling,
    /// Minor portion of version (combine with Major portion below to construct full version field)
    ///
    /// - Bytes 62-63
    pub version_minor: u16,
    /// [POSIX time](https://en.wikipedia.org/wiki/Unix_time) of last consistency check ([fsck](https://en.wikipedia.org/wiki/Fsck))
    ///
    /// - Bytes 64-67
    pub last_consistency_check: u32,
    /// Interval (in [POSIX time](https://en.wikipedia.org/wiki/Unix_time)) between forced consistency checks ([fsck](https://en.wikipedia.org/wiki/Fsck))
    ///
    /// - Bytes 68-71
    pub consistency_check_interval: u32,
    /// Operating system ID from which the filesystem on this volume was created
    ///
    /// - Bytes 72-75
    pub creator_os: OperatingSystem,
    /// Major portion of version (combine with Minor portion above to construct full version field)
    ///
    /// - Bytes 76-79
    pub version_major: u32,
    /// User ID that can use reserved blocks
    ///
    /// - Bytes 80-81
    pub user_reserved: u16,
    /// Group ID that can use reserved blocks
    ///
    /// - Bytes 82-83
    pub group_reserved: u16,

    /****************************************************************************************************************************
     * These fields are only present if Major version (specified in the base superblock fields), is greater than or equal to 1. *
     ****************************************************************************************************************************/
    /// First non-reserved inode in file system. (In versions < 1.0, this is fixed as 11)
    ///
    /// - Bytes 84-87
    pub first_non_reserved_inode: u32,
    /// Size of each inode structure in bytes. (In versions < 1.0, this is fixed as 128)
    ///
    /// - Bytes 88-89
    pub inode_size: u16,
    /// Block group that this superblock is part of (if backup copy)
    ///
    /// - Bytes 90-91
    pub block_group: u16,
    /// Optional features present (features that are not required to read or write, but usually result in a performance increase.
    ///
    /// - Bytes 92-95
    pub optional_features: OptionalFeatures,
    /// Required features present (features that are required to be supported to read or write.)
    ///
    /// - Bytes 96-99
    pub required_features: RequiredFeatures,
    /// Features that if not supported, the volume must be mounted read-only)
    ///
    /// - Bytes 100-103
    pub read_only_features: ReadOnlyFeatures,
    /// File system ID (what is output by blkid)
    ///
    /// - Bytes 104-119
    pub file_system_id: [u8; 16],
    /// Volume name (C-style string: characters terminated by a 0 byte)
    ///
    /// - Bytes 120-135
    pub volume_name: [u8; 16],
    /// Path volume was last mounted to (C-style string: characters terminated by a 0 byte)
    ///
    /// - Bytes 136-199
    pub last_mount_path: [u8; 64],
    /// Compression algorithms used (see [`Extended::required_features`])
    ///
    /// - Bytes 200-203
    pub compression_algorithms: u32,
    /// Number of blocks to preallocate for files
    ///
    /// - Bytes 204-204
    pub file_preallocation_blocks: u8,
    /// Number of blocks to preallocate for directories
    ///
    /// - Bytes 205-205
    pub directory_preallocation_blocks: u8,
    /// Unused
    ///
    /// - Bytes 206-207
    _unused_1: u16,
    /// Journal ID (same style as the File system ID)
    ///
    /// - Bytes 208-223
    pub journal_id: [u8; 16],
    /// Journal inode
    ///
    /// - Bytes 224-227
    pub journal_inode: u32,
    /// Journal device
    ///
    /// - Bytes 228-231
    pub journal_device: u32,
    /// Head of orphan inode list
    ///
    /// - Bytes 232-235
    pub orphan_inode_list_head: u32,
    /// Unused
    ///
    /// - Bytes 236-1023
    _unused_2: [u8; 788],
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Signature(u16);

impl Signature {
    pub fn valid(&self) -> bool {
        self.0 == SuperBlock::SIGNATURE
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum State {
    /// File system is clean
    Clean = 1,
    /// File system has errors
    Errors = 2,
}

#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum ErrorHandling {
    /// Ignore the error (continue on)
    Ignore = 1,
    /// Remount file system as read-only
    ReadOnly = 2,
    /// Kernel panic
    Panic = 3,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum OperatingSystem {
    /// [Linux](https://kernel.org)
    Linux = 0,
    /// [GNU HURD](https://www.gnu.org/software/hurd/hurd.html)
    GnuHurd = 1,
    /// MASIX (an operating system developed by Rémy Card, one of the developers of ext2)
    Masix = 2,
    /// [FreeBSD](https://www.freebsd.org)
    FreeBsd = 3,
    /// Other "Lites" (BSD4.4-Lite derivatives such as [NetBSD](https://www.netbsd.org), [OpenBSD](https://www.openbsd.org), [XNU/Darwin](https://www.opensource.apple.com/source/xnu), etc.)
    Other = 4,
}

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct OptionalFeatures: u32 {
        /// Preallocate some number of (contiguous?) blocks (see byte 205 in the superblock) to a directory when creating a new one (to reduce fragmentation?)
        const PREALLOCATE = 1 << 0;
        /// AFS server inodes exist
        const AFS = 1 << 1;
        /// File system has a journal (Ext3)
        const JOURNAL = 1 << 2;
        /// Inodes have extended attributes
        const EXTENDED_ATTRIBUTES = 1 << 3;
        /// File system can resize itself for larger partitions
        const RESIZABLE = 1 << 4;
        /// Directories use hash index
        const HASH = 1 << 5;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct RequiredFeatures: u32 {
        /// Compression is used
        const COMPRESSION = 1 << 0;
        /// Directory entries contain a type field
        const DIRECTORY_TYPE = 1 << 1;
        /// File system needs to replay its journal
        const REPLAY_JOURNAL = 1 << 2;
        /// File system uses a journal device
        const JOURNAL = 1 << 3;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct ReadOnlyFeatures: u32 {
        /// Sparse superblocks and group descriptor tables
        const SPARSE = 1 << 0;
        /// File system uses a 64-bit file size
        const FILE_SIZE_64 = 1 << 1;
        /// Directory contents are stored in the form of a [Binary Tree](https://en.wikipedia.org/wiki/Binary_tree)
        const BINARY_TREE = 1 << 2;
    }
}

impl SuperBlock {
    pub const SIZE: usize = 1024;
    const OFFSET: u64 = 1024;
    const SIGNATURE: u16 = 0xef53;

    /// Safety: `volume_start` must be the start address of the volume
    pub unsafe fn new(volume_start: VirtAddr) -> Option<&'static Self> {
        let superblock = &*Self::new_ptr(volume_start);
        let signature = superblock.signature;
        signature.valid().then_some(superblock)
    }

    fn new_ptr(volume_start: VirtAddr) -> *const Self {
        (volume_start + Self::OFFSET).as_ptr()
    }

    pub fn block_size(&self) -> u32 {
        1024 << self.block_size_shift
    }

    pub fn fragment_size(&self) -> u32 {
        1024 << self.fragment_size_shift
    }

    pub fn total_block_groups(&self) -> u32 {
        self.total_blocks.div_ceil(self.blocks_per_group)
    }

    /// Returns [`None`] when the block number exceeds the number of blocks defined in the superblock
    pub fn block_address(&self, number: u32) -> Option<VirtAddr> {
        if number >= self.total_blocks {
            return None;
        }

        let start_address = VirtAddr::from_ptr(self);
        let block_offset = number - self.block_number;
        let address_offset = block_offset * self.block_size();

        Some(start_address + address_offset as u64)
    }

    pub fn block_group_descriptor_table(&self) -> &BlockGroupDescriptorTable {
        let block_number = if self.block_size() == 1024 { 2 } else { 1 };
        let block_group_descriptor_table = self
            .block_address(block_number)
            .expect("block group descriptor table should be in range")
            .as_ptr();

        // Safety: The block group descriptor table is in range so its address is valid
        unsafe { &*block_group_descriptor_table }
    }

    pub fn block_group_descriptor(&self, block_group_number: u32) -> &BlockGroupDescriptor {
        &self.block_group_descriptor_table()[block_group_number as usize]
    }

    pub fn inode(&self, inode_number: u32) -> &Inode {
        let block_group_number = (inode_number - 1) / self.inodes_per_group;
        let block_group = self.block_group_descriptor(block_group_number);

        let inode_table_starting_block_address = self
            .block_address(block_group.inode_table_starting_block_number)
            .expect("inode should be in range");

        let inode_index = (inode_number - 1) % self.inodes_per_group;
        let address_offset = inode_index * self.inode_size as u32;
        let inode_address = inode_table_starting_block_address + address_offset as u64;
        let inode = inode_address.as_ptr();

        // Safety: The inode is in range so its address is valid
        unsafe { &*inode }
    }
}
