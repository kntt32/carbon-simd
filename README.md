# carbon-simd
Generic SIMD utilities for Rust, developed under the fularuen project.

## Example
```rust
use simd::*;

let mut left = [1, 2, 3, 4, 5, 6, 7, 8];
let right = [7, 6, 5, 4, 3, 2, 1, 0];
 
let mut left_simd = SimdMut::new(&mut left);
let right_simd = SimdRef::new(&right);

left_simd += &right_simd;

assert_eq!(left, [8, 8, 8, 8, 8, 8, 8, 8]);
```

## Architectures
- `x86_64`: `AVX2` is required

## Installation
Add this to your `Cargo.toml`:
```toml
[dependencies]
carbon-simd = "0.1.0"
```

## License
MIT License

