use super::BitRead;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BitCounterError {
	NotEnoughBytes,
	NotEnoughBits,
}
impl core::fmt::Display for BitCounterError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{self:?}")
	}
}
impl core::error::Error for BitCounterError {}

/// A bit counter.
/// Bit count starts at 0 and goes up when writing.
/// Bit count starts at the read limit and goes down when reading.
#[derive(Debug, Clone)]
pub struct BitCounter<S> {
	bit_stream: S,
	bit_count: usize,
}

impl<'a, S: From<&'a [u8]>> BitCounter<S> {
	pub fn new_reader(bytes: &'a [u8], bit_count_limit: usize) -> Result<Self, BitCounterError> {
		if (bytes.len() * u8::BITS as usize) < bit_count_limit {
			return Err(BitCounterError::NotEnoughBytes);
		}
		Ok(Self {
			bit_stream: S::from(bytes),
			bit_count: bit_count_limit,
		})
	}
}

impl<S: BitRead> BitRead for BitCounter<S> {
	type Output = Result<S::Output, BitCounterError>;
	fn read(&mut self, bits: usize) -> Self::Output {
		self.bit_count = self
			.bit_count
			.checked_sub(bits)
			.ok_or(BitCounterError::NotEnoughBits)?;
		Ok(self.bit_stream.read(bits))
	}
}
