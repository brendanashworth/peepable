# peepable

[![Crates.io](https://img.shields.io/crates/v/peepable.svg)](https://crates.io/crates/peepable) [![Build Status](https://travis-ci.org/brendanashworth/peepable.svg?branch=master)](https://travis-ci.org/brendanashworth/peepable) [![docs.rs](https://docs.rs/peepable/badge.svg)](https://docs.rs/peepable)

peepable is a Rust look-alike for [`Peekable`](https://doc.rust-lang.org/std/iter/struct.Peekable.html).
It behaves slightly different as it eagerly loads the next value in the Iterator.
This allows `.peep()` to be called on an immutable reference, saving you from the
borrow checker.

## Example

```rust
use std::iter::Iterator;
use peepable::Peepable;

let mut iter = vec![1, 2, 3].into_iter();

// Note, this is not "mut peeper"!
let peeper = Peepable::new(iter);

assert_eq!(peeper.peep(), Some(&1));

// When mutable, we can use it as a normal iterator.
let mut peeper = peeper;

assert_eq!(peeper.next(), Some(1));

assert_eq!(peeper.peep(), Some(&2));
assert_eq!(peeper.next(), Some(2));

assert_eq!(peeper.next(), Some(3));

assert_eq!(peeper.peep(), None);
assert_eq!(peeper.next(), None);
```

## License
peepable is licensed under the [MIT license](./LICENSE).
