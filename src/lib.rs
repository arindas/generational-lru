//! Crate providing a 100% safe, generational arena based LRUCache implementation.
//!
//! Usage:
//! ```
//! use generational_lru::lrucache::{LRUCache, CacheError};
//!
//! let capacity = 5;
//!
//! let mut lru_cache = LRUCache::<i32, i32>::with_capacity(capacity);
//! assert_eq!(lru_cache.query(&0), Err(CacheError::CacheMiss));
//!
//! for ele in 0..capacity {
//!     let x = ele as i32;
//!     assert!(lru_cache.insert(x, x).is_ok());
//! }
//!
//! for ele in 0..capacity {
//!     let x = ele as i32;
//!     assert_eq!(lru_cache.query(&x), Ok(&x));
//! }
//!
//! let x = capacity as i32;
//! assert!(lru_cache.insert(x, x).is_ok());
//!
//! assert_eq!(lru_cache.query(&x), Ok(&x));
//!
//! assert_eq!(lru_cache.query(&0), Err(CacheError::CacheMiss));
//!
//! let x = capacity as i32 / 2;
//! assert_eq!(lru_cache.remove(&x), Ok(x));
//!
//! assert_eq!(lru_cache.query(&x), Err(CacheError::CacheMiss));
//! assert_eq!(lru_cache.remove(&x), Err(CacheError::CacheMiss));
//!
//! // zero capacity LRUCache is unusable
//! let mut lru_cache = LRUCache::<i32, i32>::with_capacity(0);
//!
//! assert!(matches!(
//!     lru_cache.insert(0, 0),
//!     Err(CacheError::CacheBroken(_))
//! ));
//!
//! ```

#[forbid(unsafe_code)]

pub mod arena {
    //! Module providing a generational arena based off a vector.
    //!
    //! Usage:
    //! ```
    //! use generational_lru::arena::Arena;
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
    /// This is inspired from the crate
    /// ["generational-arena"](https://docs.rs/generational-arena)
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

    impl<T> Default for Arena<T> {
        fn default() -> Self {
            Self::new()
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
            if let Some(Entry::Occupied {
                value: _,
                generation,
            }) = self.items.get(index.idx)
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

            None
        }

        pub fn get_mut(&mut self, index: &Index) -> Option<&mut T> {
            if let Some(Entry::Occupied { value, generation }) = self.items.get_mut(index.idx) {
                if &index.generation == generation {
                    return Some(value);
                }
            }

            None
        }

        pub fn get(&self, index: &Index) -> Option<&T> {
            if let Some(Entry::Occupied { value, generation }) = self.items.get(index.idx) {
                if &index.generation == generation {
                    return Some(value);
                }
            }

            None
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

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
            assert_eq!(arena.capacity(), old_cap + to_reserve);
            assert_eq!(arena.insert(0), Err(ArenaOOM {}));
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
    //! Module providing a doubly linked list based deque implementation using a
    //! generational arena.
    //!
    //! Usage:
    //! ```
    //! use generational_lru::list::*;
    //!
    //! let capacity = 10;
    //! let mut list = LinkedList::<i32>::with_capacity(capacity);
    //! for ele in 0..capacity {
    //!     assert!(list.push_back(ele as i32).is_ok());
    //! }
    //!
    //! let mut i = 0;
    //! for ele in list.iter() {
    //!     assert_eq!(ele, &i);
    //!     i += 1;
    //! }
    //!
    //! let capacity = 10;
    //!
    //! let mut list = LinkedList::<i32>::with_capacity(capacity);
    //! assert_eq!(list.pop_front(), Err(ListError::ListEmpty));
    //!
    //! for ele in 0..capacity {
    //!     assert!(list.push_back(ele as i32).is_ok());
    //! }
    //!
    //! for ele in 0..capacity {
    //!     assert_eq!(list.pop_front().unwrap(), ele as i32);
    //! }
    //!
    //! assert!(list.is_empty());
    //! assert_eq!(list.pop_front(), Err(ListError::ListEmpty));
    //!
    //! ```

    use std::fmt::Display;

    use crate::arena::{Arena, ArenaOOM, Index};

    /// Analogous to a pointer to a Node for our generational arena list. A link
    /// uniquely refers to a node in our linked list.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Link {
        pub index: Index,
    }

    /// A Node in our linked list. It uses `Option<Link>` to point to other nodes.
    pub struct Node<T> {
        pub value: T,
        pub next: Option<Link>,
        pub prev: Option<Link>,
    }

    /// A generational arena based doubly linked list implementation.
    pub struct LinkedList<T> {
        arena: Arena<Node<T>>,

        head: Option<Link>,
        tail: Option<Link>,

        len: usize,
    }

    /// Iterator for our LinkedList.
    pub struct Iter<'a, T: 'a> {
        list: &'a LinkedList<T>,
        current: Option<Link>,
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
                ListError::ListOOM(arena_oom) => {
                    write!(f, "List out of memory: ")?;
                    arena_oom.fmt(f)
                }
                ListError::ListEmpty => write!(f, "List is empty."),
            }
        }
    }

    impl<T> Default for LinkedList<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> LinkedList<T> {
        pub fn new() -> Self {
            LinkedList {
                arena: Arena::new(),
                head: None,
                tail: None,
                len: 0,
            }
        }

        pub fn with_capacity(capacity: usize) -> Self {
            LinkedList {
                arena: Arena::with_capacity(capacity),
                head: None,
                tail: None,
                len: 0,
            }
        }

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn is_empty(&self) -> bool {
            self.head.is_none()
        }

        pub fn is_full(&self) -> bool {
            self.len == self.arena.capacity()
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

        pub fn get_mut_value(&mut self, link: &Link) -> Result<&mut T, ListError> {
            let node = self.get_mut(link)?;
            Ok(&mut node.value)
        }

        pub fn get(&self, link: &Link) -> Result<&Node<T>, ListError> {
            let node = self.arena.get(&link.index).ok_or(ListError::LinkBroken)?;
            Ok(node)
        }

        pub fn push_front(&mut self, value: T) -> Result<Link, ListError> {
            let node = Node {
                value,
                next: self.head,
                prev: None,
            };

            let index = self.arena.insert(node).map_err(ListError::ListOOM)?;
            let link = Link { index };
            if let Some(head) = self.head {
                let head_node = self.get_mut(&head)?;
                head_node.prev = Some(link);
            } else {
                self.tail = Some(link);
            }

            self.head = Some(link);

            self.len += 1;
            Ok(link)
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

            self.len += 1;
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

            self.len -= 1;
            Ok(node.value)
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

            self.len -= 1;
            Ok(node.value)
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

            self.len -= 1;
            Ok(node.value)
        }

        /// Re-arranges the nodes in the linked list to make the node pointed to by the
        /// given Link the tail node.
        pub fn reposition_to_tail(&mut self, link: &Link) -> Result<(), ListError> {
            let head = self.head.ok_or(ListError::ListEmpty)?;
            let tail = self.tail.ok_or(ListError::ListEmpty)?;

            if link == &tail {
                return Ok(());
            }

            // list has >= 2 nodes

            let head_node = self.get_mut(&head)?;
            if link == &head {
                self.head = head_node.next;
            }

            let node = self.get_mut(link)?;

            let prev_link = node.prev;
            let next_link = node.next;

            node.prev = Some(tail);
            node.next = None;

            if let Some(link) = prev_link {
                let prev = self.get_mut(&link)?;
                prev.next = next_link;
            }

            if let Some(link) = next_link {
                let next = self.get_mut(&link)?;
                next.prev = prev_link;
            }

            let tail_node = self.get_mut(&tail)?;
            tail_node.next = Some(*link);
            self.tail = Some(*link);

            Ok(())
        }

        pub fn iter(&self) -> Iter<T> {
            Iter {
                list: self,
                current: self.head(),
            }
        }
    }

    impl<'a, T: 'a> Iterator for Iter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(link) = self.current {
                if let Ok(node) = self.list.get(&link) {
                    self.current = node.next;
                    return Some(&node.value);
                }
            }

            None
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn list_new() {
            let mut list = LinkedList::<i32>::new();
            assert!(list.is_empty());
            assert!(list.is_full());

            assert_eq!(list.peek_front(), Err(ListError::ListEmpty));
            assert_eq!(list.peek_back(), Err(ListError::ListEmpty));

            assert_eq!(list.push_back(0), Err(ListError::ListOOM(ArenaOOM {})));
        }

        #[test]
        fn list_with_capacity() {
            let capacity = 5;
            let mut list = LinkedList::<i32>::with_capacity(capacity);
            assert!(list.is_empty());
            for _ in 0..capacity {
                assert!(list.push_back(0).is_ok())
            }
            assert!(list.is_full());
            assert_eq!(list.push_back(0), Err(ListError::ListOOM(ArenaOOM {})));
        }

        #[test]
        fn list_push_back() {
            let capacity = 10;
            let mut list = LinkedList::<i32>::with_capacity(capacity);
            for ele in 0..capacity {
                assert!(list.push_back(ele as i32).is_ok());
            }

            let mut i = 0;
            for ele in list.iter() {
                assert_eq!(ele, &i);
                i += 1;
            }
        }

        #[test]
        fn list_push_front() {
            let capacity = 10;
            let mut list = LinkedList::<i32>::with_capacity(capacity);
            for ele in 0..capacity {
                assert!(list.push_front(ele as i32).is_ok());
            }

            let mut i = capacity as i32 - 1;
            for ele in list.iter() {
                assert_eq!(ele, &i);
                i -= 1;
            }
        }

        #[test]
        fn list_pop_front() {
            let capacity = 10;
            let mut list = LinkedList::<i32>::with_capacity(capacity);

            assert_eq!(list.pop_front(), Err(ListError::ListEmpty));

            for ele in 0..capacity {
                assert!(list.push_back(ele as i32).is_ok());
            }

            for ele in 0..capacity {
                assert_eq!(list.pop_front().unwrap(), ele as i32);
            }

            assert!(list.is_empty());
            assert_eq!(list.pop_front(), Err(ListError::ListEmpty));
        }

        #[test]
        fn list_pop_back() {
            let capacity = 10;
            let mut list = LinkedList::<i32>::with_capacity(capacity);

            assert_eq!(list.pop_back(), Err(ListError::ListEmpty));

            for ele in 0..capacity {
                assert!(list.push_front(ele as i32).is_ok());
            }

            for ele in 0..capacity {
                assert_eq!(list.pop_back().unwrap(), ele as i32);
            }

            assert!(list.is_empty());
            assert_eq!(list.pop_back(), Err(ListError::ListEmpty));
        }

        #[test]
        fn list_remove() {
            let mut list = LinkedList::<i32>::with_capacity(5);
            assert!(list.is_empty());

            let link_0 = list.push_back(0).unwrap();
            let _link_1 = list.push_back(1).unwrap();
            let link_2 = list.push_back(2).unwrap();
            let _link_3 = list.push_back(3).unwrap();
            let link_4 = list.push_back(4).unwrap();

            assert!(list.is_full());

            assert_eq!(list.peek_front().unwrap(), &0);
            assert_eq!(list.peek_back().unwrap(), &4);

            assert!(list.remove(&link_0).is_ok());
            assert_eq!(list.len(), 4);

            assert_eq!(list.peek_front().unwrap(), &1);
            assert_eq!(list.peek_back().unwrap(), &4);

            assert!(list.remove(&link_4).is_ok());
            assert_eq!(list.len(), 3);

            assert_eq!(list.peek_front().unwrap(), &1);
            assert_eq!(list.peek_back().unwrap(), &3);

            assert!(list.remove(&link_2).is_ok());
            assert_eq!(list.len(), 2);

            assert!(list.iter().eq([1, 3].iter()));
        }

        #[test]
        fn list_reposition_to_tail() {
            let capacity = 5;

            let mut list = LinkedList::<i32>::with_capacity(capacity);
            assert!(list.is_empty());

            for ele in 0..capacity {
                list.push_back(ele as i32).unwrap();
            }

            for _ in 0..(capacity / 2) {
                list.reposition_to_tail(&list.head().unwrap()).unwrap();
            }

            let mut i = 0;
            let mut lh = 0 as i32;
            let mut rh = capacity as i32 / 2;
            for ele in list.iter() {
                if i <= (capacity / 2) {
                    assert_eq!(ele, &rh);
                    rh += 1;
                } else {
                    assert_eq!(ele, &lh);
                    lh += 1;
                }
                i += 1
            }

            let mut list = LinkedList::<i32>::with_capacity(2);
            let link_0 = list.push_back(0).unwrap();
            list.reposition_to_tail(&link_0).unwrap();
            assert_eq!(Some(link_0), list.head());
            assert_eq!(Some(link_0), list.tail());

            let link_1 = list.push_back(1).unwrap();

            list.reposition_to_tail(&link_0).unwrap();
            assert_eq!(list.get_mut_value(&link_0), Ok(&mut 0));

            assert_eq!(list.head(), Some(link_1));
            assert_eq!(list.tail(), Some(link_0));

            list.reserve(1);
            list.push_back(2).unwrap();
            list.reposition_to_tail(&link_0).unwrap();

            assert!(list.iter().eq([1, 2, 0].iter()));
        }
    }
}

pub mod lrucache {
    //! Module providing a Least-Recently-Used (LRU) Cache implementation.
    //!
    //! Usage:
    //! ```
    //! use generational_lru::lrucache::{LRUCache, CacheError};
    //!
    //! let capacity = 5;
    //!
    //! let mut lru_cache = LRUCache::<i32, i32>::with_capacity(capacity);
    //! assert_eq!(lru_cache.query(&0), Err(CacheError::CacheMiss));
    //!
    //! for ele in 0..capacity {
    //!     let x = ele as i32;
    //!     assert!(lru_cache.insert(x, x).is_ok());
    //! }
    //!
    //! for ele in 0..capacity {
    //!     let x = ele as i32;
    //!     assert_eq!(lru_cache.query(&x), Ok(&x));
    //! }
    //!
    //! let x = capacity as i32;
    //! assert!(lru_cache.insert(x, x).is_ok());
    //!
    //! assert_eq!(lru_cache.query(&x), Ok(&x));
    //!
    //! assert_eq!(lru_cache.query(&0), Err(CacheError::CacheMiss));
    //!
    //! let x = capacity as i32 / 2;
    //! assert_eq!(lru_cache.remove(&x), Ok(x));
    //!
    //! assert_eq!(lru_cache.query(&x), Err(CacheError::CacheMiss));
    //! assert_eq!(lru_cache.remove(&x), Err(CacheError::CacheMiss));
    //!
    //! // zero capacity LRUCache is unusable
    //! let mut lru_cache = LRUCache::<i32, i32>::with_capacity(0);
    //!
    //! assert!(matches!(
    //!     lru_cache.insert(0, 0),
    //!     Err(CacheError::CacheBroken(_))
    //! ));
    //!
    //! ```

    use crate::list::{Link, LinkedList, ListError};
    use std::{collections::HashMap, fmt::Display, hash::Hash};

    /// Cache block storing some key and value.
    pub struct Block<K, V> {
        pub key: K,
        pub value: V,
    }

    /// A Least-Recently-Used (LRU) Cache implemented using a generational arena
    /// based linked list and a hash map.
    pub struct LRUCache<K, V>
    where
        K: Eq + Hash,
    {
        blocks: LinkedList<Block<K, V>>,
        block_refs: HashMap<K, Link>,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum CacheError {
        CacheBroken(ListError),
        CacheMiss,
    }

    impl Display for CacheError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self {
                CacheError::CacheBroken(list_error) => {
                    write!(f, "Cache storage is broken: ")?;
                    list_error.fmt(f)
                }
                CacheError::CacheMiss => write!(f, "Key not found in cache."),
            }
        }
    }

    impl<K, V> LRUCache<K, V>
    where
        K: Eq + Hash + Copy,
    {
        /// Creates an LRUCache instance with the given capacity. A zero capacity LRUCache is
        /// unusable.
        pub fn with_capacity(capacity: usize) -> Self {
            LRUCache {
                blocks: LinkedList::with_capacity(capacity),
                block_refs: HashMap::new(),
            }
        }

        /// Returns a reference to the value associated with the given key. If the key is not
        /// present in the cache, we return a "cache-miss" error. If the entry is found but
        /// cannot be fetched from the underlying storage, we return a "cache-broken" error.
        pub fn query(&mut self, key: &K) -> Result<&V, CacheError> {
            let link = self.block_refs.get(key).ok_or(CacheError::CacheMiss)?;
            self.blocks
                .reposition_to_tail(link)
                .map_err(CacheError::CacheBroken)?;
            let node = self.blocks.get(link).map_err(CacheError::CacheBroken)?;
            Ok(&node.value.value)
        }

        /// Removes the associated key value pair for the given key from this cache. If no
        /// entry is found, we return a "cache-miss" error. If the entry is found but cannot
        /// be fetched from the underlying in-memory storage, we return a "cache-broken" error.
        /// Returns the value associated, after removal with ownership.
        pub fn remove(&mut self, key: &K) -> Result<V, CacheError> {
            let link = self.block_refs.remove(key).ok_or(CacheError::CacheMiss)?;
            let block = self.blocks.remove(&link).map_err(CacheError::CacheBroken)?;
            Ok(block.value)
        }

        /// Inserts a new key value pair into this cache. If this cache is full, the least
        /// recently used entry is removed.
        pub fn insert(&mut self, key: K, value: V) -> Result<(), CacheError> {
            if let Some(link) = self.block_refs.get(&key) {
                self.blocks
                    .reposition_to_tail(link)
                    .map_err(CacheError::CacheBroken)?;
                let block_ref = self
                    .blocks
                    .get_mut_value(link)
                    .map_err(CacheError::CacheBroken)?;
                block_ref.value = value;
                return Ok(());
            }

            if self.blocks.is_full() {
                let block = self.blocks.pop_front().map_err(CacheError::CacheBroken)?;
                self.block_refs.remove(&block.key);
            }

            let link = self
                .blocks
                .push_back(Block { key, value })
                .map_err(CacheError::CacheBroken)?;
            self.block_refs.insert(key, link);

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn lru_cache_consistency() {
            let mut lru_cache = LRUCache::<i32, i32>::with_capacity(0);
            assert_eq!(
                lru_cache.insert(0, 0),
                Err(CacheError::CacheBroken(ListError::ListEmpty))
            );

            let mut lru_cache = LRUCache::<i32, i32>::with_capacity(2);
            assert!(lru_cache.insert(1, 1).is_ok());
            assert!(lru_cache.insert(2, 2).is_ok());
            assert_eq!(lru_cache.query(&1), Ok(&1));
            assert!(lru_cache.insert(3, 3).is_ok());
            assert_eq!(lru_cache.query(&2), Err(CacheError::CacheMiss));
            assert!(lru_cache.insert(1, -1).is_ok());
            assert_eq!(lru_cache.query(&1), Ok(&-1));
            assert!(lru_cache.insert(4, 4).is_ok());
            assert_eq!(lru_cache.query(&3), Err(CacheError::CacheMiss));

            let capacity = 5;

            let mut lru_cache = LRUCache::<i32, i32>::with_capacity(capacity);
            assert_eq!(lru_cache.query(&0), Err(CacheError::CacheMiss));

            for ele in 0..capacity {
                let x = ele as i32;
                assert!(lru_cache.insert(x, x).is_ok());
            }

            for ele in 0..capacity {
                let x = ele as i32;
                assert_eq!(lru_cache.query(&x), Ok(&x));
            }

            let x = capacity as i32;
            assert!(lru_cache.insert(x, x).is_ok());

            assert_eq!(lru_cache.query(&x), Ok(&x));

            assert_eq!(lru_cache.query(&0), Err(CacheError::CacheMiss));

            let x = capacity as i32 / 2;
            assert_eq!(lru_cache.remove(&x), Ok(x));

            assert_eq!(lru_cache.query(&x), Err(CacheError::CacheMiss));
            assert_eq!(lru_cache.remove(&x), Err(CacheError::CacheMiss));
        }
    }
}
