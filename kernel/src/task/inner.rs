use elf::endian::NativeEndian;
use elf::ElfBytes;

pub struct Task<'data> {
    elf: ElfBytes<'data, NativeEndian>,
}

impl<'data> Task<'data> {
    pub fn new(elf: ElfBytes<'data, NativeEndian>) -> Self {
        Self { elf }
    }
}
