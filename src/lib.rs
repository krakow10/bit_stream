#![no_std]

/// Buffer size for read and write
type Cache = usize;

pub trait BitRead {
	type Output;
	fn read(&mut self, bits: usize) -> Self::Output;
}

mod end_array_chunks;

mod bit_counter;
pub use bit_counter::*;

mod bit_reader_le;
pub use bit_reader_le::*;
pub type CountedBitReaderLe<'a> = BitCounter<BitReaderLe<'a>>;

mod bit_reader_be;
pub use bit_reader_be::*;
pub type CountedBitReaderBe<'a> = BitCounter<BitReaderBe<'a>>;

// test readme
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
struct ReadmeDoctests;
