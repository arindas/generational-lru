pub mod arena {
    //! Module providing a generational arena based off a vector.
    //!
    //! Usage:
    //! ```
    //! use lrucache::arena::Arena;
    //!
    //! let mut arena = Arena::<i32>::with_capacity(10); // create arena
    //! let index = arena.insert(78).unwrap(); // allocate new element in arena
    //! let i_ref = arena.get(&index);
    //! assert_eq!(i_ref, Some(&78));
    //! let i_m_ref = arena.get_mut(&index).unwrap();
    //! *i_m_ref = -68418; // this close from greatness
    //! assert_eq!(arena.get(&index), Some(&-68418));
    //!
    //! arena.remove(&index).unwrap();
    //!
    //! assert!(arena.get(&index).is_none());
    //! ```

    use std::fmt::Display;

    /// Index in vector to allocated entry. Used to access items allocated in
    /// the arena.
    #[derive(Debug, PartialEq)]
    pub struct Index {
        pub idx: usize,
        pub generation: u64,
    }

    /// Entry represents an arena allocation entry. It is used to track free
    /// and Occupied blocks along with generation counters for Occupied
    /// blocks.
    #[derive(Debug, PartialEq)]
    pub enum Entry<T> {
        Free { next_free: Option<usize> },
        Occupied { value: T, generation: u64 },
    }

    /// A generational arena for allocating memory based off a vector. Every
    /// entry is associated with a generation counter to uniquely identify
    /// newer allocations from older reclaimed allocations at the same
    /// position in the vector.
    /// This is inspired from the crate "generational-arena" on cargo.rs
    pub struct Arena<T> {
        items: Vec<Entry<T>>,
        capacity: usize,

        generation: u64,

        free_list_head: Option<usize>,
    }

    /// Arena out of memory error.
    #[derive(Debug, Clone, PartialEq)]
    pub struct ArenaOOM;

    impl Display for ArenaOOM {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Arena out of memory.")
        }
    }

    impl<T> Arena<T> {
        pub fn new() -> Self {
            Arena {
                items: Vec::new(),
                capacity: 0,
                generation: 0,
                free_list_head: None,
            }
        }

        pub fn capacity(&self) -> usize {
            self.capacity
        }

        pub fn reserve(&mut self, capacity: usize) {
            self.items.reserve_exact(capacity);
            let start = self.items.len();
            let end = start + capacity;
            let old_free = self.free_list_head;
            self.items.extend((start..end).map(|i| {
                if i == end - 1 {
                    Entry::Free {
                        next_free: old_free,
                    }
                } else {
                    Entry::Free {
                        next_free: Some(i + 1),
                    }
                }
            }));
            self.free_list_head = Some(start);
            self.capacity += capacity;
        }

        pub fn with_capacity(capacity: usize) -> Self {
            let mut arena = Self::new();
            arena.reserve(capacity);
            arena
        }

        pub fn insert(&mut self, item: T) -> Result<Index, ArenaOOM> {
            if self.free_list_head.is_none() {
                return Err(ArenaOOM {});
            }

            let old_free = self.free_list_head;
            if let Entry::Free { next_free } = self.items[old_free.unwrap()] {
                self.free_list_head = next_free;
            } else {
                return Err(ArenaOOM {});
            }

            let entry = Entry::Occupied {
                value: item,
                generation: self.generation,
            };
            self.items[old_free.unwrap()] = entry;
            self.generation += 1;

            Ok(Index {
                idx: old_free.unwrap(),
                generation: self.generation - 1,
            })
        }

        pub fn remove(&mut self, index: &Index) -> Option<()> {
            if let Some(entry) = self.items.get(index.idx) {
                if let Entry::Occupied {
                    value: _,
                    generation,
                } = entry
                {
                    if &index.generation != generation {
                        return None;
                    }

                    let entry = Entry::<T>::Free {
                        next_free: self.free_list_head,
                    };
                    self.items[index.idx] = entry;
                    self.free_list_head = Some(index.idx);
                    return Some(());
                }
            }

            None
        }

        pub fn get_mut(&mut self, index: &Index) -> Option<&mut T> {
            if let Some(entry) = self.items.get_mut(index.idx) {
                if let Entry::Occupied { value, generation } = entry {
                    if &index.generation == generation {
                        return Some(value);
                    }
                }
            }

            None
        }

        pub fn get(&self, index: &Index) -> Option<&T> {
            if let Some(entry) = self.items.get(index.idx) {
                if let Entry::Occupied { value, generation } = entry {
                    if &index.generation == generation {
                        return Some(value);
                    }
                }
            }

            None
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_works() {
            let result = 2 + 2;
            assert_eq!(result, 4);
        }

        #[test]
        fn arena_new() {
            Arena::<i32>::new();
        }

        #[test]
        fn arena_with_capacity() {
            let capacity = 100;
            let arena = Arena::<i32>::with_capacity(capacity);
            assert_eq!(arena.capacity(), capacity);

            assert_eq!(arena.free_list_head, Some(0));
            let mut i = 0;
            for entry in &arena.items {
                if i == capacity - 1 {
                    assert_eq!(entry, &Entry::Free { next_free: None })
                } else {
                    assert_eq!(
                        entry,
                        &Entry::Free {
                            next_free: Some(i + 1)
                        }
                    )
                }

                i += 1;
            }
        }

        #[test]
        fn arena_insert() {
            let mut arena = Arena::<i32>::new();
            assert_eq!(arena.insert(0), Err(ArenaOOM {}));

            arena.reserve(1);
            let index_0 = arena.insert(0);
            assert_eq!(
                index_0,
                Ok(Index {
                    idx: 0,
                    generation: 0
                })
            );

            arena.reserve(1);
            let index_1 = arena.insert(1);
            assert_eq!(
                index_1,
                Ok(Index {
                    idx: 1,
                    generation: 1
                })
            );

            let index_0_val = index_0.unwrap();
            let item_0 = arena.get(&index_0_val);
            assert_eq!(item_0, Some(&0));

            let index_1_val = index_1.unwrap();
            let item_1 = arena.get(&index_1_val);
            assert_eq!(item_1, Some(&1));

            let item_0 = arena.get_mut(&index_0_val);
            assert_eq!(item_0, Some(&mut 0));
            let item_0 = item_0.unwrap();
            *item_0 = 25;

            let item_0 = arena.get(&index_0_val);
            assert_eq!(item_0, Some(&25));

            let item_1 = arena.get_mut(&index_1_val);
            assert_eq!(item_1, Some(&mut 1));
            let item_1 = item_1.unwrap();
            *item_1 = -78;

            let item_1 = arena.get(&index_1_val);
            assert_eq!(item_1, Some(&-78));

            assert_eq!(arena.capacity(), 2);
            assert_eq!(arena.insert(0), Err(ArenaOOM {}));

            let old_cap = arena.capacity();
            let to_reserve = 100;
            arena.reserve(to_reserve);
            for ele in 0..to_reserve {
                assert_eq!(
                    arena.insert(0),
                    Ok(Index {
                        idx: old_cap + ele,
                        generation: (old_cap + ele) as u64
                    })
                )
            }
        }

        #[test]
        fn arena_remove() {
            let mut arena = Arena::<i32>::with_capacity(1);

            let index = arena.insert(0).unwrap();
            assert_eq!(arena.get(&index), Some(&0));

            arena.remove(&index).unwrap();

            assert_eq!(arena.get(&index), None);

            let index = arena.insert(0).unwrap();
            assert_eq!(
                index,
                Index {
                    idx: 0,
                    generation: 1
                }
            );

            arena.remove(&index).unwrap();
            assert!(arena.remove(&index).is_none());

            let current_gen = 2;

            let to_reserve = 5;
            arena.reserve(to_reserve);
            for ele in 0..to_reserve + 1 {
                // free list head moves forward. list circles back to start
                if ele == to_reserve {
                    assert_eq!(
                        arena.insert(0),
                        Ok(Index {
                            idx: 0,
                            generation: (current_gen + ele) as u64
                        })
                    )
                } else {
                    assert_eq!(
                        arena.insert(0),
                        Ok(Index {
                            idx: ele + 1,
                            generation: (current_gen + ele) as u64
                        })
                    )
                }
            }
        }
    }
}
