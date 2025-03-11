#[derive(Debug)]
#[repr(C, packed)]
pub struct DirectoryEntry {
    /// Inode
    ///
    /// - Bytes 0-3
    pub inode: u32,
    /// Total size of this entry (Including all subfields)
    ///
    /// - Bytes 4-5
    pub size: u16,
    /// Name Length least-significant 8 bits
    ///
    /// - Bytes 6-6
    pub name_length_low: u8,
    /// Type indicator (only if the feature bit for "directory entries have file type byte" is set, else this is the most-significant 8 bits of the Name Length)
    ///
    /// - Bytes 7-7
    pub type_indicator: DirectoryEntryType,
    /// Name characters
    ///
    /// - Bytes 8-N
    pub name: &'static [u8],
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum DirectoryEntryType {
    /// Unknown type
    Unknown = 0,
    /// Regular file
    File = 1,
    /// Directory
    Directory = 2,
    /// Character device
    Character = 3,
    /// Block device
    Block = 4,
    /// FIFO
    Fifo = 5,
    /// Socket
    Socket = 6,
    /// Symbolic link (soft link)
    Symlink = 7,
}
