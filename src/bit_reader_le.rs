use crate::end_array_chunks::end_array_chunks;

use super::{BitRead, Cache};

pub struct BitReaderLe<'a> {
	chunks: core::slice::Iter<'a, [u8; size_of::<Cache>()]>,
	cache: Cache,
	cache_bits: usize,
}
impl<'a> BitReaderLe<'a> {
	pub fn new(bytes: &'a [u8]) -> Self {
		let (rem, chunks) = end_array_chunks(bytes);

		let mut chunk = [0; _];
		chunk[..rem.len()].copy_from_slice(rem);

		Self {
			chunks: chunks.iter(),
			cache: Cache::from_le_bytes(chunk),
			cache_bits: rem.len() * u8::BITS as usize,
		}
	}
}
impl<'a> From<&'a [u8]> for BitReaderLe<'a> {
	fn from(value: &'a [u8]) -> Self {
		Self::new(value)
	}
}
impl<'a> BitRead for BitReaderLe<'a> {
	type Output = Cache;
	fn read(&mut self, bits: usize) -> Cache {
		debug_assert!(bits <= Cache::BITS as usize);

		let mut value = 0;
		let mut value_bits = 0;

		// popluate cache with enough bits to fill value
		while self.cache_bits + value_bits < bits {
			value |= self.cache.unbounded_shl(value_bits as u32);
			value_bits += self.cache_bits;

			self.cache = match self.chunks.next() {
				Some(chunk) => Cache::from_le_bytes(*chunk),
				None => 0,
			};
			self.cache_bits = Cache::BITS as usize;
		}

		// populate value with cached bits
		let draw_bits = bits - value_bits;
		let mask = (1 as Cache).unbounded_shl(draw_bits as u32).wrapping_sub(1);
		value |= (self.cache & mask).unbounded_shl(value_bits as u32);
		self.cache = self.cache.unbounded_shr(draw_bits as u32);
		self.cache_bits -= draw_bits;
		value
	}
}

#[test]
fn test_read_bytes() {
	let mut r = BitReaderLe::new(b"asdf");
	assert_eq!(r.read(8), 'a' as Cache);
	assert_eq!(r.read(8), 's' as Cache);
	assert_eq!(r.read(8), 'd' as Cache);
	assert_eq!(r.read(8), 'f' as Cache);
	// end of bytes
	assert_eq!(r.read(0), 0);
	assert_eq!(r.read(1), 0);
}

#[test]
fn test_read_bits() {
	fn assert_s(shift: usize) {
		assert_eq!(
			BitReaderLe::new(b"s").read(shift),
			's' as Cache & ((1 as Cache).unbounded_shl(shift as u32) - 1)
		);
	}
	assert_s(0);
	assert_s(1);
	assert_s(2);
	assert_s(3);
	assert_s(4);
	assert_s(5);
	assert_s(6);
	assert_s(7);
	assert_s(8);
}

#[test]
fn test_read_sequence() {
	let mut r = BitReaderLe::new(b"asd");
	// dsa 011_00100 011_1001_1 011_00_001
	assert_eq!(r.read(3), 0b001);
	assert_eq!(r.read(2), 0b00);
	assert_eq!(r.read(4), 0b1_011);
	assert_eq!(r.read(4), 0b1001);
	assert_eq!(r.read(8), 0b00100_011);
	assert_eq!(r.read(3), 0b011);
	// end of bytes
	assert_eq!(r.read(0), 0);
	assert_eq!(r.read(1), 0);
}
