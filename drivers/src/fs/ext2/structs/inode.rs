use bitflags::bitflags;

use super::block_pointer::BlockPointers;

#[derive(Debug)]
#[repr(C, packed)]
pub struct Inode {
    /// Type and Permissions
    ///
    /// - Bytes 0-1
    pub type_permissions: TypePermissions,
    /// User ID
    ///
    /// - Bytes 2-3
    pub user_id: u16,
    /// Lower 32 bits of size in bytes
    ///
    /// - Bytes 4-7
    pub size_low: u32,
    /// Last Access Time (in [POSIX time](https://en.wikipedia.org/wiki/Unix_time))
    ///
    /// - Bytes 8-11
    pub last_access: u32,
    /// Creation Time (in [POSIX time](https://en.wikipedia.org/wiki/Unix_time))
    ///
    /// - Bytes 12-15
    pub creation_time: u32,
    /// Last Modification time (in [POSIX time](https://en.wikipedia.org/wiki/Unix_time))
    ///
    /// - Bytes 16-19
    pub last_modification: u32,
    /// Deletion time (in [POSIX time](https://en.wikipedia.org/wiki/Unix_time))
    ///
    /// - Bytes 20-23
    pub deletion_time: u32,
    /// Group ID
    ///
    /// - Bytes 24-25
    pub group_id: u16,
    /// Count of hard links (directory entries) to this inode. When this reaches 0, the data blocks are marked as unallocated.
    ///
    /// - Bytes 26-27
    pub hard_links: u16,
    /// Count of disk sectors (not Ext2 blocks) in use by this inode, not counting the actual inode structure nor directory entries linking to the inode.
    ///
    /// - Bytes 28-31
    pub disk_sectors: u32,
    /// Flags
    ///
    /// - Bytes 32-35
    pub flags: Flags,
    /// Operating System Specific value #1
    ///
    /// - Bytes 36-39
    pub os_specific_1: [u8; 4],
    /// Block Pointers
    ///
    /// - Bytes 40-99
    pub block_pointers: BlockPointers,
    /// Generation number (Primarily used for NFS)
    ///
    /// - Bytes 100-103
    pub generation_number: u32,
    /// In Ext2 version 0, this field is reserved. In version >= 1, Extended attribute block (File ACL).
    ///
    /// - Bytes 104-107
    pub file_acl: u32,
    /// In Ext2 version 0, this field is reserved. In version >= 1, Upper 32 bits of file size (if feature bit set) if it's a file, Directory ACL if it's a directory
    ///
    /// - Bytes 108-111
    pub size_upper_or_directory_acl: u32,
    /// Block address of fragment
    ///
    /// - Bytes 112-115
    pub fragment_block_number: u32,
    /// Operating System Specific Value #2
    ///
    /// - Bytes 116-127
    pub os_specific_2: [u8; 12],
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct TypePermissions {
    pub type_permissions: u16,
}

#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum Type {
    /// FIFO
    Fifo = 0x1000,
    /// Character device
    Character = 0x2000,
    /// Directory
    Directory = 0x4000,
    /// Block device
    Block = 0x6000,
    /// Regular file
    File = 0x8000,
    /// Symbolic link
    Symlink = 0xA000,
    /// Unix socket
    Socket = 0xC000,
}

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct Permissions: u16 {
        /// [Other - execute permission](https://en.wikipedia.org/wiki/Filesystem_permissions#Traditional_Unix_permissions)
        const OTHER_EXECUTE = 1 << 0;
        /// [Other - write permission](https://en.wikipedia.org/wiki/Filesystem_permissions#Traditional_Unix_permissions)
        const OTHER_WRITE = 1 << 1;
        /// [Other - read permission](https://en.wikipedia.org/wiki/Filesystem_permissions#Traditional_Unix_permissions)
        const OTHER_READ = 1 << 2;
        /// [Group - execute permission](https://en.wikipedia.org/wiki/Filesystem_permissions#Traditional_Unix_permissions)
        const GROUP_EXECUTE = 1 << 3;
        /// [Group - write permission](https://en.wikipedia.org/wiki/Filesystem_permissions#Traditional_Unix_permissions)
        const GROUP_WRITE = 1 << 4;
        /// [Group - read permission](https://en.wikipedia.org/wiki/Filesystem_permissions#Traditional_Unix_permissions)
        const GROUP_READ = 1 << 5;
        /// [User - execute permission](https://en.wikipedia.org/wiki/Filesystem_permissions#Traditional_Unix_permissions)
        const USER_EXECUTE = 1 << 6;
        /// [User - write permission](https://en.wikipedia.org/wiki/Filesystem_permissions#Traditional_Unix_permissions)
        const USER_WRITE = 1 << 7;
        /// [User - read permission](https://en.wikipedia.org/wiki/Filesystem_permissions#Traditional_Unix_permissions)
        const USER_READ = 1 << 8;
        /// [Sticky Bit](https://en.wikipedia.org/wiki/Sticky_bit)
        const STICKY_BIT = 1 << 9;
        /// Set group ID
        const SET_GROUP_ID = 1 << 10;
        /// Set user ID
        const SET_USER_ID = 1 << 11;
    }
}

impl TypePermissions {
    pub fn split(&self) -> (Type, Permissions) {
        let file_type = self.type_permissions & 0xF000;
        let permissions = self.type_permissions & 0x0FFF;

        // Safety: The variables are created above
        let file_type = unsafe { *(file_type as *const Type) };
        let permissions = unsafe { *(permissions as *const Permissions) };

        (file_type, permissions)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct Flags: u32 {
        /// Secure deletion (not used)
        const SECURE_DELETION = 1 << 0;
        /// Keep a copy of data when deleted (not used)
        const KEEP_WHEN_DELETED = 1 << 1;
        /// File compression (not used)
        const FILE_COMPRESSION = 1 << 2;
        /// Synchronous updates — new data is written immediately to disk
        const SYNCHRONOUS_UPDATES = 1 << 3;
        /// Immutable file (content cannot be changed)
        const IMMUTABLE_FILE = 1 << 4;
        /// Append only
        const APPEND_ONLY = 1 << 5;
        /// File is not included in 'dump' command
        const NOT_INCLUDED_IN_DUMP = 1 << 6;
        /// Last accessed time should not updated
        const NO_LAST_ACCESS_UPDATE = 1 << 7;

        /**
         * [...] Reserved [...]
         */

        /// Hash indexed directory
        const HASH_INDEXED_DIRECTORY = 1 << 16;
        /// AFS directory
        const AFS_DIRECTORY = 1 << 17;
        /// Journal file data
        const JOURNAL_FILE_DATA = 1 << 18;
    }
}

impl Inode {
    pub fn file_type(&self) -> Type {
        let type_permissions = self.type_permissions;
        type_permissions.split().0
    }

    pub fn permissions(&self) -> Permissions {
        let type_permissions = self.type_permissions;
        type_permissions.split().1
    }
}
