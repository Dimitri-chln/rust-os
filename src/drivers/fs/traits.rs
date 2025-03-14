use crate::utils::posix::path::PathBuf;

pub trait FileSystem {
    type File;

    fn read<'a>(
        &'a self,
        path: PathBuf,
        current_directory: Option<&'a Self::File>,
    ) -> Option<&'a Self::File>;
}
