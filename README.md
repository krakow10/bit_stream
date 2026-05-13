Bit Stream
==========

[![Latest version](https://img.shields.io/crates/v/bit_stream.svg)](https://crates.io/crates/bit_stream)
![License](https://img.shields.io/crates/l/bit_stream.svg)

`bit_stream` is intended to be a collection of low-level adapters for reading and writing bits, but currently only includes readers.

Curently includes:
- BitReaderLe
- BitReaderBe
- CountedBitReaderLe
- CountedBitReaderBe

## Examples

```rust
use bit_stream::BitReaderLe;
let mut r = BitReaderLe::new(b"asdf");
assert_eq!(r.read_le(8) as u8, 'a' as u8);
assert_eq!(r.read_le(8) as u8, 's' as u8);
assert_eq!(r.read_le(8) as u8, 'd' as u8);
// 'f' = 0b01_100_110 read lsb first (right to left)
assert_eq!(r.read_le(3), 0b110);
assert_eq!(r.read_le(3), 0b100);
assert_eq!(r.read_le(2), 0b01);
```

```rust
use bit_stream::BitReaderBe;
let mut r = BitReaderBe::new(b"asdf");
assert_eq!(r.read_be(8) as u8, 'a' as u8);
assert_eq!(r.read_be(8) as u8, 's' as u8);
assert_eq!(r.read_be(8) as u8, 'd' as u8);
// 'f' = 0b01_100_110 read msb first (left to right)
assert_eq!(r.read_be(2), 0b01);
assert_eq!(r.read_be(3), 0b100);
assert_eq!(r.read_be(3), 0b110);
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
