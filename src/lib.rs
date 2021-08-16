use bytes::Bytes;
use linked_hash_map::LinkedHashMap;
use std::collections::VecDeque;

pub struct LRU {
    items: LinkedHashMap<String, Bytes>,
    max_size: usize,
    current_size: usize,
    history: VecDeque<String>,
    accesses: usize,
    hits: usize,
}

impl LRU {
    pub fn new() -> Self {
        LRU {
            items: LinkedHashMap::new(),
            max_size: 64,
            accesses: 0,
            hits: 0,
            current_size: 0,
            history: VecDeque::with_capacity(1000),
        }
    }
    ///
    /// Builder for max_size, only outside-configurable value for cache
    ///
    /// ```
    /// use lfu_vecs::LFU;
    /// let lfu = LFU::new().max_size(1024);
    /// ```
    ///
    pub fn max_size(mut self, size: usize) -> Self {
        self.max_size = size;
        self
    }

    pub fn current_size(self) -> usize {
        self.current_size
    }

    pub fn has_evicted_recently(&self, key: &str) -> bool {
        self.history
            .iter()
            .any(|historical_key| historical_key.eq(key))
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.items.contains_key(key)
    }

    pub fn insert(&mut self, key: String, value: Bytes) -> Option<Bytes> {
        // TODO: implement eviction code
        while self.current_size + value.len() > self.max_size {
            if let Some((popped_key, popped_item)) = self.items.pop_front() {
                self.current_size -= popped_item.len();
                // register deletion in history
                if self.history.len() > self.max_size {
                    self.history.pop_back();
                }
                self.history.push_front(popped_key);
            }
        }
        // add and increment
        self.current_size += value.len();
        self.items.insert(key, value)
    }
    pub fn get(&mut self, key: &str) -> Option<&Bytes> {
        self.accesses += 1;
        if self.items.contains_key(key) {
            self.hits += 1;
        }
        self.items.get(key)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use bytes::Bytes;

    #[test]
    fn no_evictions() {
        let mut lru = LRU::new();
        lru = lru.max_size(128);
        lru.insert("a".to_owned(), Bytes::from("a"));
        assert_eq!(lru.get("a"), Some(&Bytes::from("a")));
    }

    #[test]
    fn test_evictions_history() {
        let mut lru = LRU::new();
        lru = lru.max_size(5); // smaller by one from what we will ask for
        lru.insert("a".to_owned(), Bytes::from("abc"));
        lru.insert("b".to_owned(), Bytes::from("dfg"));
        assert_eq!(lru.get("a"), None);
        assert_eq!(lru.history, vec!["a"]);
        assert_eq!(lru.has_evicted_recently("a"), true);
    }
}
