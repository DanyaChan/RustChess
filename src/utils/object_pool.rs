use std::cell::RefCell;

pub struct ObjectPool<T> {
    cur: usize,
    objects: Vec<Vec<T>>,
}

impl<T> ObjectPool<T> {
    pub fn new(n: usize) -> Self {
        let mut ret = ObjectPool {
            cur: 0,
            objects: Vec::with_capacity(n),
        };
        ret.objects.push(Vec::with_capacity(50));
        return ret;
    }

    pub fn get(&mut self) -> RefCell<&mut Vec<T>> {
        if self.cur >= self.objects.len() {
            self.objects.push(Vec::with_capacity(50));
        }
        self.cur += 1;
        return RefCell::new(&mut self.objects[self.cur - 1]);
    }
}