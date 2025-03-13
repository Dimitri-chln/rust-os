use x86_64::VirtAddr;

use super::structs::directory_entry;
use super::structs::inode::{Inode, Type};
use super::structs::superblock::SuperBlock;
use crate::utils::posix::path::PathBuf;

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

    pub fn root_inode(&self) -> &Inode {
        self.superblock.inode(2)
    }

    pub fn read<'a>(
        &'a self,
        path: PathBuf,
        current_directory: Option<&'a Inode>,
    ) -> Option<&'a Inode> {
        let mut current_inode = current_directory.unwrap_or(self.root_inode());
        let mut path_iter = path.iter();

        while let Some(part) = path_iter.next() {
            let Some(mut entries) = self.read_directory(current_inode) else {
                break;
            };
            let Some(entry) = entries.find(|entry| entry.name() == part) else {
                return None;
            };

            current_inode = self.superblock.inode(entry.inode);
        }

        path_iter.next().is_none().then_some(current_inode)
    }

    pub fn read_directory<'a>(&'a self, inode: &'a Inode) -> Option<directory_entry::Iter<'a>> {
        let type_permissions = inode.type_permissions;
        let (file_type, _) = type_permissions.split();

        match file_type {
            Type::Directory => unsafe {
                // Safety: We just checked that the inode was a directory
                Some(inode.block_pointers.iter_directory_entries(self.superblock))
            },
            _ => None,
        }
    }
}
