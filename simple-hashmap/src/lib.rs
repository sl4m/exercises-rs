use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

macro_rules! map {
    ( $( $x:expr ),* ) => {
        {
            let mut map = Hashmap::new();
            $(
                map.insert($x.0, $x.1);
            )*
            map
        }
    }
}

const HASH_SIZE: usize = 16;

pub struct Hashmap<K: Hash + Eq, V> {
    slots: [Vec<(K, V)>; HASH_SIZE],
}

impl<K: Hash + Eq, V> Hashmap<K, V> {
    pub fn new() -> Hashmap<K, V> {
        Self {
            slots: Default::default(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let slot_index = Self::get_index_hash_pair(&key);
        match self.get_entry_index(slot_index, &key) {
            Some(entry_index) => {
                let old_tuple =
                    std::mem::replace(&mut self.slots[slot_index][entry_index], (key, value));
                Some(old_tuple.1)
            }
            None => {
                self.slots[slot_index].push((key, value));
                None
            }
        }
    }

    pub fn get(&self, key: K) -> Option<&V> {
        if let Some((slot_index, entry_index)) = self.get_indices(&key) {
            Some(&self.slots[slot_index][entry_index].1)
        } else {
            None
        }
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        if let Some((slot_index, entry_index)) = self.get_indices(&key) {
            let removed_tuple = self.slots[slot_index].remove(entry_index);
            Some(removed_tuple.1)
        } else {
            None
        }
    }

    fn get_indices(&self, key: &K) -> Option<(usize, usize)> {
        let slot_index = Self::get_index_hash_pair(&key);
        if let Some(entry_index) = self.get_entry_index(slot_index, &key) {
            Some((slot_index, entry_index))
        } else {
            None
        }
    }

    fn get_entry_index(&self, slot_index: usize, lookup_key: &K) -> Option<usize> {
        self.slots[slot_index]
            .iter()
            .enumerate()
            .find_map(|(i, tuple)| {
                if tuple.0 == *lookup_key {
                    Some(i)
                } else {
                    None
                }
            })
    }

    fn get_index_hash_pair(key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hashed_key = hasher.finish();
        (hashed_key % HASH_SIZE as u64) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_inserts_key_value_pair() {
        let mut map = Hashmap::new();
        assert_eq!(map.insert("foo", "bar"), None);
    }

    #[test]
    fn it_inserts_new_value_for_existing_key_value_pair() {
        let mut map = Hashmap::new();
        map.insert("foo", "bar");
        assert_eq!(map.insert("foo", "lol"), Some("bar"));
    }

    #[test]
    fn it_gets_the_value_for_existing_key_value_pair() {
        let mut map = Hashmap::new();
        map.insert("foo", "bar");
        assert_eq!(map.get("foo"), Some(&"bar"));
        assert_eq!(map.get("foo"), Some(&"bar"));
    }

    #[test]
    fn it_returns_none_for_non_existing_key_value_pair() {
        let mut map = Hashmap::new();
        map.insert("foo", "bar");
        assert_eq!(map.get("qux"), None);
    }

    #[test]
    fn it_removes_existing_key_value_pair() {
        let mut map = Hashmap::new();
        map.insert("foo", "bar");
        assert_eq!(map.remove("foo"), Some("bar"));
        assert_eq!(map.get("foo"), None);
    }

    #[test]
    fn it_works_with_another_type() {
        let mut map = Hashmap::new();
        assert_eq!(map.insert(1, 3), None);
        assert_eq!(map.insert(1, 4), Some(3));
        assert_eq!(map.get(1), Some(&4));
        assert_eq!(map.get(2), None);
        assert_eq!(map.remove(1), Some(4));
        assert_eq!(map.get(1), None);
    }

    #[test]
    fn it_creates_a_hashmap_using_macro() {
        let map = map![("foo", "bar"), ("baz", "buz")];
        assert_eq!(map.get("foo"), Some(&"bar"));
        assert_eq!(map.get("baz"), Some(&"buz"));
    }
}
