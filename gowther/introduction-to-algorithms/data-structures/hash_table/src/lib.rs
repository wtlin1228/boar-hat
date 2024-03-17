//! Simple implementation for HashTable with chaining

#![feature(linked_list_remove)]

use rand::Rng;
use std::collections::LinkedList;

#[derive(Debug)]
struct ListNode<K, V> {
    key: K,
    value: V,
}

pub struct HashTable<K, V>
where
    K: Key,
{
    size: usize,
    pointers: Vec<Option<LinkedList<ListNode<K, V>>>>,
    m: usize,
    p: usize,
    a: usize,
}

pub trait Key {
    fn to_usize(&self) -> usize;
}

impl<K, V> HashTable<K, V>
where
    K: Key,
{
    pub fn new() -> Self {
        let m = 2;
        let mut pointers = Vec::with_capacity(m);
        pointers.resize_with(m, || None);
        let p = 2usize.pow(31) - 1;
        let mut rng = rand::thread_rng();

        Self {
            size: 0,
            pointers,
            m,
            p,
            a: rng.gen_range(0..p),
        }
    }

    fn hash(&self, k: usize) -> usize {
        // ((a * k) mod p) mod m
        ((self.a * k) % self.p) % self.m
    }

    fn resize(&mut self, n: usize) {
        if n < self.m {
            return;
        }

        let prev_m = self.m;
        self.m *= 2;
        let mut pointers: Vec<Option<LinkedList<ListNode<K, V>>>> = Vec::with_capacity(self.m);
        pointers.resize_with(self.m, || None);
        for i in 0..prev_m {
            if let Some(mut list) = self.pointers[i].take() {
                while list.len() > 0 {
                    let node = list.pop_front().expect("pop one node failed");
                    let hash = self.hash(node.key.to_usize());
                    match pointers.get_mut(hash).unwrap() {
                        Some(list) => list.push_back(node),
                        None => {
                            let mut list = LinkedList::new();
                            list.push_back(node);
                            pointers[hash] = Some(list);
                        }
                    }
                }
            }
        }

        self.pointers = pointers;
    }

    pub fn find(&self, key: K) -> Option<&V> {
        let hash = self.hash(key.to_usize());
        match self.pointers.get(hash).unwrap() {
            None => None,
            Some(list) => {
                for node in list.iter() {
                    if key.to_usize() == node.key.to_usize() {
                        return Some(&node.value);
                    }
                }
                None
            }
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.resize(self.size + 1);
        let hash = self.hash(key.to_usize());
        let node = ListNode { key, value };
        match self.pointers.get_mut(hash).unwrap() {
            Some(list) => list.push_back(node),
            None => {
                let mut list = LinkedList::new();
                list.push_back(node);
                self.pointers[hash] = Some(list);
            }
        }
        self.size += 1;
    }

    pub fn delete(&mut self, key: K) -> Option<V> {
        let is_delete_target = |node: &ListNode<K, V>| node.key.to_usize() == key.to_usize();

        let hash = self.hash(key.to_usize());
        match self.pointers.get_mut(hash).unwrap() {
            None => None,
            Some(list) => {
                let mut i = 0;
                for node in list.iter() {
                    if is_delete_target(node) {
                        self.size -= 1;
                        return Some(list.remove(i).value);
                    }
                    i += 1;
                }
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Key for usize {
        fn to_usize(&self) -> usize {
            *self
        }
    }

    #[test]
    fn it_works() {
        let mut hash_table: HashTable<usize, String> = HashTable::new();

        hash_table.insert(488, "Leo".to_string());
        hash_table.insert(32, "Una".to_string());
        hash_table.insert(341, "Kirby".to_string());

        assert_eq!(hash_table.find(341), Some(&"Kirby".to_string()));
        assert_eq!(hash_table.find(488), Some(&"Leo".to_string()));
        assert_eq!(hash_table.find(32), Some(&"Una".to_string()));
        assert_eq!(hash_table.find(99981), None);
        assert_eq!(hash_table.find(0), None);

        assert_eq!(hash_table.delete(341), Some("Kirby".to_string()));
        assert_eq!(hash_table.delete(488), Some("Leo".to_string()));
        assert_eq!(hash_table.delete(32), Some("Una".to_string()));
        assert_eq!(hash_table.delete(99981), None);
        assert_eq!(hash_table.delete(0), None);

        assert_eq!(hash_table.find(341), None);
        assert_eq!(hash_table.find(488), None);
        assert_eq!(hash_table.find(32), None);
        assert_eq!(hash_table.find(99981), None);
        assert_eq!(hash_table.find(0), None);
    }
}
