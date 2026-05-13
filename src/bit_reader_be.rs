use crate::end_array_chunks::end_array_chunks;

use super::{BitRead, Cache};

/// Read bits from the slice in order. Bits are read as if
/// from each byte, starting from the most significant bit.
pub struct BitReaderBe<'a> {
	chunks: core::slice::Iter<'a, [u8; size_of::<Cache>()]>,
	cache: Cache,
	cache_bits: usize,
}
impl<'a> BitReaderBe<'a> {
	pub fn new(bytes: &'a [u8]) -> Self {
		let (rem, chunks) = end_array_chunks(bytes);

		let mut chunk = [0; _];
		chunk[size_of::<Cache>() - rem.len()..].copy_from_slice(rem);

		Self {
			chunks: chunks.iter(),
			cache: Cache::from_be_bytes(chunk),
			cache_bits: rem.len() * u8::BITS as usize,
		}
	}
}
impl<'a> BitRead for BitReaderBe<'a> {
	type Output = Cache;
	fn read(&mut self, bits: usize) -> Cache {
		debug_assert!(bits <= Cache::BITS as usize);

		let mut value: Cache = 0;
		let mut value_bits = 0;

		// popluate cache with enough bits to fill value
		while self.cache_bits + value_bits < bits {
			value = value.unbounded_shl(self.cache_bits as u32) | self.cache;
			value_bits += self.cache_bits;

			self.cache = self.chunks.next().copied().map_or(0, Cache::from_be_bytes);
			self.cache_bits = Cache::BITS as usize;
		}

		// populate value with cached bits
		let draw_bits = bits - value_bits;
		let mask = (1 as Cache).unbounded_shl(draw_bits as u32).wrapping_sub(1);
		value = (value << draw_bits) | (self.cache >> (self.cache_bits - draw_bits));
		self.cache &= !mask.unbounded_shl((self.cache_bits - draw_bits) as u32);
		self.cache_bits -= draw_bits;
		value
	}
}

#[test]
fn test_read_bytes() {
	let mut r = BitReaderBe::new(b"asdf");
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
			BitReaderBe::new(b"s").read(shift),
			('s' as u8 & (1u8.unbounded_shl(shift as u32) - 1).reverse_bits())
				.unbounded_shr(u8::BITS - shift as u32) as Cache
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
	let mut r = BitReaderBe::new(b"asd");
	// --> reading bits from left to right
	// asd 011_00_001 011_1001_1 011_00100
	assert_eq!(r.read(3), 0b011);
	assert_eq!(r.read(2), 0b00);
	assert_eq!(r.read(6), 0b001_011);
	assert_eq!(r.read(4), 0b1001);
	assert_eq!(r.read(4), 0b1_011);
	assert_eq!(r.read(5), 0b00100);
	// end of bytes
	assert_eq!(r.read(0), 0);
	assert_eq!(r.read(1), 0);
}
