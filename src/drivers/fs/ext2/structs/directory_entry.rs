use alloc::borrow::Cow;
use core::slice;

use encoding_rs::WINDOWS_1252;

use super::{
    block_pointer::{self, BlockPointers, DirectPointer},
    superblock::SuperBlock,
};

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
    // /// Name characters (ISO-Latin-1 in most cases)
    // ///
    // /// - Bytes 8-N
    // pub name: &'a [u8],
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

impl DirectoryEntry {
    pub fn name(&self) -> Cow<str> {
        let start_ptr = self as *const Self;
        let ptr_offset = size_of::<Self>();
        let name_ptr = unsafe { start_ptr.add(ptr_offset) as *const u8 };
        let name_bytes = unsafe { slice::from_raw_parts(name_ptr, self.name_length_low as usize) };
        let (name, _) = WINDOWS_1252.decode_without_bom_handling(name_bytes);

        name
    }

    fn next_ptr(&self) -> *const DirectoryEntry {
        let start_ptr = self as *const Self;

        unsafe { start_ptr.add(self.size as usize) }
    }
}

pub struct Iter<'a> {
    superblock: &'a SuperBlock,
    data_block_pointer_iter: block_pointer::Iter<'a>,
    inner_iter: Option<InnerIter<'a>>,
}

impl<'a> Iter<'a> {
    pub fn new(data_block_pointers: &'a BlockPointers, superblock: &'a SuperBlock) -> Self {
        let mut data_block_pointer_iter = data_block_pointers.iter(superblock);
        let inner_iter = data_block_pointer_iter
            .next()
            .map(|data_block_number| InnerIter::new(data_block_number, superblock));

        Self {
            superblock,
            data_block_pointer_iter,
            inner_iter,
        }
    }

    fn next_data_block(&mut self) {
        self.inner_iter = self
            .data_block_pointer_iter
            .next()
            .map(|data_block_number| InnerIter::new(data_block_number, self.superblock));
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a DirectoryEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(ref mut inner_iter) = self.inner_iter else {
            return None;
        };

        inner_iter.next().or_else(|| {
            self.next_data_block();
            self.next()
        })
    }
}

struct InnerIter<'a> {
    superblock: &'a SuperBlock,
    data_block_start_ptr: *const u8,
    next: Option<&'a DirectoryEntry>,
}

impl<'a> InnerIter<'a> {
    pub fn new(data_block_number: DirectPointer, superblock: &'a SuperBlock) -> Self {
        let data_block_start_ptr = superblock
            .block_ptr(*data_block_number)
            .expect("data block out of range");

        let directory_entry_ptr = data_block_start_ptr as *const DirectoryEntry;
        let directory_entry = unsafe { &*directory_entry_ptr };

        Self {
            superblock,
            data_block_start_ptr,
            next: Some(directory_entry),
        }
    }
}

impl<'a> Iterator for InnerIter<'a> {
    type Item = &'a DirectoryEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(directory_entry) = self.next else {
            return None;
        };

        let next_ptr = directory_entry.next_ptr();
        let next_ptr_offset = next_ptr.addr() - self.data_block_start_ptr.addr();
        let is_ptr_valid = next_ptr_offset < self.superblock.block_size() as usize;
        self.next = is_ptr_valid.then_some(unsafe { &*next_ptr });

        return Some(directory_entry);
    }
}
