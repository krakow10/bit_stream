use crate::BitRead;
use crate::bit_buffer::{BitBuffer, Cache};
use crate::end_array_chunks::end_array_chunks;

/// Read bits from the slice in order. Bits are read as if
/// from each byte, starting from the least significant bit.
#[derive(Debug, Clone)]
pub struct BitReaderLe<'a> {
	chunks: core::slice::Iter<'a, [u8; size_of::<Cache>()]>,
	cache: BitBuffer,
}
impl<'a> BitReaderLe<'a> {
	pub fn new(bytes: &'a [u8]) -> Self {
		let (rem, chunks) = end_array_chunks(bytes);

		let mut chunk = [0; _];
		chunk[..rem.len()].copy_from_slice(rem);

		Self {
			chunks: chunks.iter(),
			cache: BitBuffer::new(Cache::from_le_bytes(chunk), rem.len() * u8::BITS as usize),
		}
	}
	pub fn read_le(&mut self, bits: usize) -> Cache {
		debug_assert!(bits <= Cache::BITS as usize);

		let mut value = BitBuffer::empty();

		// popluate cache with enough bits to fill value
		while self.cache.bits() + value.bits() < bits {
			// This won't work if cache is a different size from value
			let (buffer, bits) = self.cache.take();
			value.push_msb(bits, buffer);

			self.cache = BitBuffer::new(
				self.chunks.next().copied().map_or(0, Cache::from_le_bytes),
				Cache::BITS as usize,
			);
		}

		// populate value with cached bits
		let draw_bits = bits - value.bits();
		value.push_msb(draw_bits, self.cache.pop_lsb(draw_bits));
		value.value()
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
		self.read_le(bits)
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
	// reading bits from right to left <--
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
