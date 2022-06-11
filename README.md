# lrucache

Crate providing a 100% safe, generational arena based LRU cache implementation.

Usage:
```
use lrucache::lrucache::{LRUCache, CacheError};

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

