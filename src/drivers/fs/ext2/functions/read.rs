use crate::{
    drivers::fs::ext2::{
        structs::inode::{Inode, Type},
        System,
    },
    utils::posix::path::PathBuf,
};

impl System {
    pub fn read_from_root(&self, path: PathBuf) -> &Inode {
        self.read_from_inode(path, self.root_inode())
    }

    pub fn read_from_inode<'a>(&self, path: PathBuf, source: &'a Inode) -> &'a Inode {
        let mut inode = source;

        for part in path.iter() {
            let type_permissions = inode.type_permissions;
            let (file_type, _) = type_permissions.split();

            match file_type {
                Type::Directory => {
                    for data_block_number in inode.block_pointers.iter(&self.superblock) {
                        let data_block = self
                            .superblock
                            .block(*data_block_number)
                            .expect("data block out of range");
                    }
                }
                file_type => {}
            }
        }

        inode
    }
}
