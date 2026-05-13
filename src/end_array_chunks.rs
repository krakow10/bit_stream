/// Array chunks which are aligned to the end of the slice.
/// The remainder is taken from the beginning of the slice.
pub const fn end_array_chunks<const CHUNK_SIZE: usize, T>(
	slice: &[T],
) -> (&[T], &[[T; CHUNK_SIZE]]) {
	let (chunks, rem) = (slice.len() / CHUNK_SIZE, slice.len() % CHUNK_SIZE);

	// SAFETY: 0 <= rem <= slice.len() by construction above
	let (begin, rest) = unsafe { slice.split_at_unchecked(rem) };

	let ptr = rest.as_ptr().cast();
	// SAFETY: chunks * CHUNK_SIZE == slice.len() - rem
	let array_chunks = unsafe { core::slice::from_raw_parts(ptr, chunks) };

	(begin, array_chunks)
}
