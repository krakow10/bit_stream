type Cache = usize;

#[derive(Debug)]
pub enum BitReaderError {
	NotEnoughBytes,
}

mod bit_reader_le;
pub use bit_reader_le::BitReaderLe;
