# lrucache
[![ci](https://github.com/arindas/generational-lru/actions/workflows/ci.yml/badge.svg)](https://github.com/arindas/generational-lru/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/arindas/generational-lru/branch/main/graph/badge.svg?token=W2BBX6MPW8)](https://codecov.io/gh/arindas/lrucache)
[![rustdoc](https://github.com/arindas/generational-lru/actions/workflows/rustdoc.yml/badge.svg)](https://github.com/arindas/generational-lru/actions/workflows/rustdoc.yml)

Crate providing a 100% safe, generational arena based LRU cache implementation.

```rust
use generational_lru::lrucache::{LRUCache, CacheError};

let capacity = 5;

let mut lru_cache = LRUCache::<i32, i32>::with_capacity(capacity);
assert_eq!(lru_cache.get(&0), Err(CacheError::CacheMiss));

for ele in 0..capacity {
    let x = ele as i32;
    assert!(lru_cache.insert(x, x).is_ok());
}

for ele in 0..capacity {
    let x = ele as i32;
    assert_eq!(lru_cache.get(&x), Ok(&x));
}

let x = capacity as i32;
assert!(lru_cache.insert(x, x).is_ok());

assert_eq!(lru_cache.get(&x), Ok(&x));

assert_eq!(lru_cache.get(&0), Err(CacheError::CacheMiss));

let x = capacity as i32 / 2;
assert_eq!(lru_cache.remove(&x), Ok(x));

assert_eq!(lru_cache.get(&x), Err(CacheError::CacheMiss));
assert_eq!(lru_cache.remove(&x), Err(CacheError::CacheMiss));

// zero capacity LRUCache is unusable
let mut lru_cache = LRUCache::<i32, i32>::with_capacity(0);

assert!(matches!(
    lru_cache.insert(0, 0),
    Err(CacheError::CacheBroken(_))
));

```

Refer to [API documentation](https://arindas.github.io/generational-lru/generational_lru) for more details.

## Usage
This is a library crate. You may include it in your `Cargo.toml` as follows:
```toml
[dependencies]
generational-lru = "0.1.0"
```

## License
This repository is licensed under the MIT License. See [LICENSE](./LICENSE) for the full license text.
