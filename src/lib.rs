/// Buffer size for read and write
type Cache = usize;

pub trait BitRead {
	type Output;
	fn read(&mut self, bits: usize) -> Self::Output;
}

mod end_array_chunks;

mod bit_reader_le;
pub use bit_reader_le::BitReaderLe;

mod bit_counter;
pub use bit_counter::BitCounter;
pub type CountedBitReaderLe<'a> = BitCounter<BitReaderLe<'a>>;
