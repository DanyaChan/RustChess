pub struct StoredObjects<T> {
    object: Vec<T>,
    taken: bool,
}
pub struct ObjectPool<T> {
    last_available: i32,
    objects: Vec<StoredObjects<T>>,
}

pub struct ReturnedValue<'a, T> {
    index: usize,
    pub value: &'a mut Vec<T>
}

// no thread safety
impl<'a, T> ObjectPool<T> {
    pub fn new(num: usize, size: usize) -> Self {
        let mut ret = ObjectPool {
            objects: Vec::with_capacity(num),
            last_available: num as i32 - 1,
        };
        for i in 0..num {
            ret.objects.push(StoredObjects {
                object: Vec::with_capacity(size),
                taken: false,
            });
        }
        return ret;
    }

    pub fn get(&mut self) -> ReturnedValue<'a, T> {
        assert!(self.last_available >= 0);
        let index = self.last_available as usize;
        self.objects[self.last_available as usize].taken = true;
        while self.last_available >= 0 && self.objects[self.last_available as usize].taken {
            self.last_available -= 1;
        }
        unsafe {
            let ptr : *mut Vec<T> = &mut self.objects[index].object;
            return ReturnedValue {
                index,
                value: &mut *ptr,
            };
        };
    }

    pub fn return_back(&mut self, resource: ReturnedValue<'a, T>) {
        self.objects[resource.index].taken = false;
        self.last_available = resource.index as i32;
    }
}