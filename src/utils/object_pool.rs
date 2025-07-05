use std::cell::RefCell;

pub struct ObjectPool<T, const N: usize> {
    available: usize,
    objects: [Vec<T>; N],
}

impl<T, const N: usize> ObjectPool<T, N> {
    pub fn new() -> Self {
        return ObjectPool<T, N> {

        }
    }
    pub fn get(&self) -> RefCell<Vec<T>> {
        if self.available == N {
            panic!()
        }
        return Resourse { idx: , resourse: () }
        
    }
}