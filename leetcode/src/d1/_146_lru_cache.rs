use std::collections::{HashMap, LinkedList};
use std::fmt::Debug;

#[derive(Debug)]
struct LRUCache {
    capacity: i32,
    cache: HashMap<i32, i32>,
    order: LinkedList<i32>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity,
            cache: HashMap::new(),
            order: LinkedList::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&v) = self.cache.get(&key) {
            // Remove key from its current position in order
            let mut cursor = self.order.cursor_front_mut();
            while let Some(&mut k) = cursor.current() {
                if k == key {
                    cursor.remove_current();
                    break;
                }
                cursor.move_next();
            }
            // Mark as most recently used by pushing to front
            self.order.push_front(key);
            v
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.cache.contains_key(&key) {
            // Key exists: update value and move to front
            self.cache.insert(key, value);
            // Remove key from its current position in order
            let mut cursor = self.order.cursor_front_mut();
            while let Some(&mut k) = cursor.current() {
                if k == key {
                    cursor.remove_current();
                    break;
                }
                cursor.move_next();
            }
            self.order.push_front(key);
        } else {
            // Key doesn’t exist: check capacity and insert
            if self.cache.len() as i32 >= self.capacity {
                // Cache full: remove least recently used key
                if let Some(oldest) = self.order.pop_back() {
                    self.cache.remove(&oldest);
                }
            }
            // Insert new key-value pair
            self.cache.insert(key, value);
            self.order.push_front(key);
        }
    }
}

#[test]
fn test() {
    let mut obj = LRUCache::new(2);
    assert_eq!(obj.get(2), -1);
    obj.put(2, 6);
    assert_eq!(obj.get(1), -1);
    obj.put(1, 5);
    obj.put(1, 2);
    assert_eq!(obj.get(1), 2);
    assert_eq!(obj.get(2), 6);

    let mut obj = LRUCache::new(3);
    obj.put(1, 1);
    obj.put(2, 2);
    obj.put(3, 3);
    obj.put(4, 4);
    assert_eq!(obj.get(4), 4);
    assert_eq!(obj.get(3), 3);
    assert_eq!(obj.get(2), 2);
    assert_eq!(obj.get(1), -1);
    obj.put(5, 5);
    assert_eq!(obj.get(1), -1);
    assert_eq!(obj.get(2), 2);
    assert_eq!(obj.get(3), 3);
    assert_eq!(obj.get(4), -1);
    assert_eq!(obj.get(5), 5);
}
