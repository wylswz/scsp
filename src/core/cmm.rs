use std::borrow::Borrow;
use std::{borrow::BorrowMut, collections::HashMap, hash::Hash, sync::Arc};

use std::sync::{Mutex as Mux, MutexGuard};


#[derive(Debug)]
pub struct ConcurrentMultiMap<K, V: Send> {
    m: Mux<HashMap<K, Mux<Vec<V>>>>
}

impl<K, V> ConcurrentMultiMap<K, V> where 
    K: Eq + Hash + Clone,
    V: Send {
    
    pub fn new() -> Self {
        ConcurrentMultiMap{
            m: Mux::new(HashMap::new())
        }
    }
    
    pub fn append(&mut self, k: K, v: V) {
        self.touch(k.clone());
        self.do_append(k, v)
    }

    pub fn for_each(&mut self, k: K, callback: impl Fn(&V)) {
        self.with_key(&k, |v| {
            for item in v.iter() {
                callback(&item);
            }
        })
    }

    fn with_key(&mut self, k: &K, callback: impl Fn(MutexGuard<Vec<V>>)) {
        let _ = self.m.borrow_mut().lock().map(|m| {
            m.get(k).map(Mux::lock).map(|lr| {
                 let _ = lr.map( |inner_vec| {
                    callback(inner_vec);
                 }); 
            });
        });
    }

    fn do_append(&mut self, k: K, v: V) {
        let _ = self.m.borrow_mut().lock().map(|inner| {
            inner.get(&k).map(Mux::lock).map(|f| {
                f.map(|mut inner_vec| {
                    inner_vec.push(v);
                })
            });
        });
    }

    fn touch(&mut self, k: K) {
        let _ = self.m.borrow_mut().lock().map(|mut inner| {
            if !inner.contains_key(&k) {
                inner.insert(k, Mux::new(vec![]));
            }
        });
    }

}


#[test]
fn test_append() {
    let mut m = ConcurrentMultiMap::<&str, i64>::new();
    m.append("k1", 1);
    m.append("k1", 2);

}