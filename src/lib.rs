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
    #[derive(Debug, PartialEq, Clone, Copy)]
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

        pub fn remove(&mut self, index: &Index) -> Option<T> {
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

                    let old_entry = core::mem::replace(&mut self.items[index.idx], entry);

                    self.free_list_head = Some(index.idx);

                    if let Entry::Occupied {
                        value,
                        generation: _,
                    } = old_entry
                    {
                        return Some(value);
                    }
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

            assert_eq!(arena.remove(&index).unwrap(), 0);

            assert_eq!(arena.get(&index), None);

            let index = arena.insert(56).unwrap();
            assert_eq!(
                index,
                Index {
                    idx: 0,
                    generation: 1
                }
            );

            assert_eq!(arena.remove(&index).unwrap(), 56);
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

pub mod list {
    use std::fmt::Display;

    use crate::arena::{Arena, ArenaOOM, Index};

    #[derive(Clone, Copy, PartialEq)]
    pub struct Link {
        pub index: Index,
    }

    pub struct Node<T> {
        pub value: T,
        pub next: Option<Link>,
        pub prev: Option<Link>,
    }

    pub struct LinkedList<T> {
        arena: Arena<Node<T>>,

        head: Option<Link>,
        tail: Option<Link>,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum ListError {
        LinkBroken,
        ListOOM(ArenaOOM),
        ListEmpty,
    }

    impl Display for ListError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self {
                ListError::LinkBroken => write!(f, "Link does not point to a valid location."),
                ListError::ListOOM(e) => e.fmt(f),
                ListError::ListEmpty => write!(f, "List is empty."),
            }
        }
    }

    impl<T> LinkedList<T> {
        pub fn with_capacity(capacity: usize) -> Self {
            LinkedList {
                arena: Arena::with_capacity(capacity),
                head: None,
                tail: None,
            }
        }

        pub fn reserve(&mut self, capacity: usize) {
            self.arena.reserve(capacity)
        }

        pub fn get_mut(&mut self, link: &Link) -> Result<&mut Node<T>, ListError> {
            let node = self
                .arena
                .get_mut(&link.index)
                .ok_or(ListError::LinkBroken)?;
            Ok(node)
        }

        pub fn get(&self, link: &Link) -> Result<&Node<T>, ListError> {
            let node = self.arena.get(&link.index).ok_or(ListError::LinkBroken)?;
            Ok(node)
        }

        pub fn push_back(&mut self, value: T) -> Result<Link, ListError> {
            let node = Node {
                value,
                next: None,
                prev: self.tail,
            };

            let index = self.arena.insert(node).map_err(ListError::ListOOM)?;
            let link = Link { index };
            if let Some(tail) = self.tail {
                let tail_node = self.get_mut(&tail)?;
                tail_node.next = Some(link);
            } else {
                self.head = Some(link)
            }

            self.tail = Some(link);

            Ok(link)
        }

        pub fn head(&self) -> Option<Link> {
            self.head
        }

        pub fn tail(&self) -> Option<Link> {
            self.tail
        }

        pub fn peek_front(&self) -> Result<&T, ListError> {
            let head_link = self.head.ok_or(ListError::ListEmpty)?;
            return self.get(&head_link).map(|x| &x.value);
        }

        pub fn peek_back(&self) -> Result<&T, ListError> {
            let tail_link = self.tail.ok_or(ListError::ListEmpty)?;
            return self.get(&tail_link).map(|x| &x.value);
        }

        pub fn pop_front(&mut self) -> Result<T, ListError> {
            let head_link = self.head.ok_or(ListError::ListEmpty)?;
            let node = self
                .arena
                .remove(&head_link.index)
                .ok_or(ListError::LinkBroken)?;

            self.head = node.next;

            if let Some(link) = self.head {
                let cur_head_node = self.get_mut(&link)?;
                cur_head_node.prev = None;
            } else {
                self.tail = None;
            }

            return Ok(node.value);
        }

        pub fn pop_back(&mut self) -> Result<T, ListError> {
            let tail_link = self.tail.ok_or(ListError::ListEmpty)?;
            let node = self
                .arena
                .remove(&tail_link.index)
                .ok_or(ListError::LinkBroken)?;

            self.tail = node.prev;
            if let Some(link) = self.tail {
                let cur_tail_node = self.get_mut(&link)?;
                cur_tail_node.next = None;
            } else {
                self.head = None;
            }

            return Ok(node.value);
        }

        pub fn remove(&mut self, link: &Link) -> Result<T, ListError> {
            let head = self.head.ok_or(ListError::ListEmpty)?;
            let tail = self.tail.ok_or(ListError::ListEmpty)?;

            if link == &head {
                return self.pop_front();
            }

            if link == &tail {
                return self.pop_back();
            }

            let node = self
                .arena
                .remove(&link.index)
                .ok_or(ListError::LinkBroken)?;
            let prev_link = node.prev.ok_or(ListError::LinkBroken)?;
            let next_link = node.next.ok_or(ListError::LinkBroken)?;

            let prev = self.get_mut(&prev_link)?;
            prev.next = Some(next_link);

            let next = self.get_mut(&next_link)?;
            next.prev = Some(prev_link);

            return Ok(node.value);
        }
    }

    #[cfg(test)]
    mod tests {

        #[test]
        fn it_works() {
            let result = 2 + 2;
            assert_eq!(result, 4);
        }
    }
}
