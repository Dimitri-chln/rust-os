use core::{
    array,
    iter::{Chain, Flatten},
};

use super::{block::Block, superblock::SuperBlock};

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct BlockPointers {
    /// Direct Block Pointers
    pub direct: [DirectPointer; 12],
    /// Singly Indirect Block Pointer (Points to a block that is a list of block pointers to data)
    pub singly_indirect: SinglyIndirect,
    /// Doubly Indirect Block Pointer (Points to a block that is a list of block pointers to Singly Indirect Blocks)
    pub doubly_indirect: DoublyIndirect,
    /// Triply Indirect Block Pointer (Points to a block that is a list of block pointers to Doubly Indirect Blocks)
    pub triply_indirect: TriplyIndirect,
}

impl BlockPointers {
    pub fn iter<'a>(&'a self, superblock: &'a SuperBlock) -> Iter<'a> {
        Iter::new(self, superblock)
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct DirectPointer(u32);

impl core::ops::Deref for DirectPointer {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct SinglyIndirect(u32);

struct SinglyIndirectIter<'a> {
    block: Block<'a>,
    next: usize,
}

impl<'a> SinglyIndirectIter<'a> {
    fn new(singly_indirect: SinglyIndirect, superblock: &'a SuperBlock) -> Self {
        let block_ptr = superblock
            .block_ptr(singly_indirect.0)
            .expect("singly indirect block out of range");

        let block = Block::from_ptr(block_ptr, superblock.block_size() as usize);
        Self { block, next: 0 }
    }
}

impl<'a> Iterator for SinglyIndirectIter<'a> {
    type Item = DirectPointer;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next >= self.block.len() {
            return None;
        }

        let block_number = u32::from_le_bytes(
            self.block[self.next..(self.next + size_of::<u32>())]
                .try_into()
                .unwrap(),
        );

        self.next += size_of::<u32>();
        Some(DirectPointer(block_number))
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct DoublyIndirect(u32);

struct DoublyIndirectIter<'a> {
    superblock: &'a SuperBlock,
    block: Block<'a>,
    next: usize,
}

impl<'a> DoublyIndirectIter<'a> {
    fn new(doubly_indirect: DoublyIndirect, superblock: &'a SuperBlock) -> Self {
        let block_ptr = superblock
            .block_ptr(doubly_indirect.0)
            .expect("doubly indirect block out of range");

        let block = Block::from_ptr(block_ptr, superblock.block_size() as usize);
        Self {
            superblock,
            block,
            next: 0,
        }
    }
}

impl<'a> Iterator for DoublyIndirectIter<'a> {
    type Item = SinglyIndirectIter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next >= self.block.len() {
            return None;
        }

        let singly_indirect = SinglyIndirect(u32::from_le_bytes(
            self.block[self.next..(self.next + size_of::<u32>())]
                .try_into()
                .unwrap(),
        ));

        self.next += size_of::<u32>();
        Some(SinglyIndirectIter::new(singly_indirect, self.superblock))
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct TriplyIndirect(u32);

struct TriplyIndirectIter<'a> {
    superblock: &'a SuperBlock,
    block: Block<'a>,
    next: usize,
}

impl<'a> TriplyIndirectIter<'a> {
    fn new(triply_indirect: TriplyIndirect, superblock: &'a SuperBlock) -> Self {
        let block_ptr = superblock
            .block_ptr(triply_indirect.0)
            .expect("triply indirect block out of range");

        let block = Block::from_ptr(block_ptr, superblock.block_size() as usize);
        Self {
            superblock,
            block,
            next: 0,
        }
    }
}

impl<'a> Iterator for TriplyIndirectIter<'a> {
    type Item = DoublyIndirectIter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next >= self.block.len() {
            return None;
        }

        let doubly_indirect = DoublyIndirect(u32::from_le_bytes(
            self.block[self.next..(self.next + size_of::<u32>())]
                .try_into()
                .unwrap(),
        ));

        self.next += size_of::<u32>();
        Some(DoublyIndirectIter::new(doubly_indirect, self.superblock))
    }
}

type DirectIter = array::IntoIter<DirectPointer, 12>;
type SinglyIter<'a> = SinglyIndirectIter<'a>;
type DoublyIter<'a> = Flatten<DoublyIndirectIter<'a>>;
type TriplyIter<'a> = Flatten<Flatten<TriplyIndirectIter<'a>>>;

pub struct Iter<'a> {
    superblock: &'a SuperBlock,
    inner: Chain<Chain<Chain<DirectIter, SinglyIter<'a>>, DoublyIter<'a>>, TriplyIter<'a>>,
}

impl<'a> Iter<'a> {
    fn new(value: &BlockPointers, superblock: &'a SuperBlock) -> Self {
        let direct_pointers = value.direct;
        let direct_iter = direct_pointers.into_iter();
        // Safety: Indirect values are read from the inode
        let singly_iter = SinglyIndirectIter::new(value.singly_indirect, superblock);
        let doubly_iter = DoublyIndirectIter::new(value.doubly_indirect, superblock);
        let triply_iter = TriplyIndirectIter::new(value.triply_indirect, superblock);

        Self {
            superblock,
            inner: direct_iter
                .chain(singly_iter)
                .chain(doubly_iter.flatten())
                .chain(triply_iter.flatten().flatten()),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = DirectPointer;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
