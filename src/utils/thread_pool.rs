use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::thread::*;

use std::sync::{Mutex, Arc};
pub struct Task<Data, Result> {
    pub task: fn(Data) -> Result,
    pub data: Data,
}



struct ThreadPool<Data, Result> {
    tasks_queue: VecDeque<Task<Data, Result>>,
    result_queue: VecDeque<Result>,
    tasks_in_progress: usize,
    // threads_handles: Vec<JoinHandle<()>>,
    stop: bool
}

pub struct ThreadPoolWrap<Data, Result>{
    thread_pool: Arc<Mutex<ThreadPool<Data, Result>>>,
}
fn thread_run_cycle<Data, Result>(thread_pool: Arc<Mutex<ThreadPool<Data, Result>>>) {
    loop {
        let task = {
            let lock = thread_pool.lock();
            if lock.is_err() {
                continue;
            }
            let mut pool_lock = lock.unwrap();
            let pool = pool_lock.deref_mut();
            if pool.stop {
                return;
            }
            if pool.tasks_queue.is_empty() {
                continue;
            }
            pool.tasks_in_progress+=1;
            pool.tasks_queue.pop_front().unwrap()
        };
        let res = (task.task)(task.data);
        let lock = thread_pool.lock();
        if lock.is_err() {
            continue;
        }
        let mut pool_lock = lock.unwrap();
        let pool = pool_lock.deref_mut();
        pool.tasks_in_progress-=1;
        pool.result_queue.push_back(res);
    }
}

impl<Data, Result> ThreadPool<Data, Result> where Data: 'static + Send + Clone, Result: 'static + Send + Clone {
    pub fn new(threads: usize) -> Arc<Mutex<Self>> {
        let ret = Arc::new(Mutex::new( Self{
            tasks_queue: VecDeque::new(),
            result_queue: VecDeque::new(),
            tasks_in_progress: 0,
            // threads_handles: Vec::new(),
            stop: false,
        }));


        for _ in 0..threads {
            let ret_c = ret.clone();
            spawn(move || thread_run_cycle(ret_c));
        }
        ret.clone()
    }

}

impl<Data, Result> ThreadPoolWrap<Data, Result> where Data: 'static + Send + Clone, Result: 'static + Send + Clone {
    pub fn new(threads: usize) -> Self {
         Self {
            thread_pool: ThreadPool::new(threads),
        }
    }
    pub fn await_all_tasks(&self) {
        loop {
            let lock = self.thread_pool.lock();
            if lock.is_err() {
                continue;
            }
            let pool_lock = lock.unwrap();
            let pool = pool_lock.deref();
            if pool.tasks_in_progress == 0 && pool.tasks_queue.is_empty() {
                break;
            }
        }
    }
    pub fn get_all_tasks_and_clear(&self) -> VecDeque<Result> {
        let lock = self.thread_pool.lock();
        let mut pool_lock = lock.expect("Failed to acquire mutex in get_all_tasks_and_clear");
        let pool = pool_lock.deref_mut();
        let ret = pool.result_queue.clone();
        pool.result_queue.clear();
        ret
    }

    pub fn put_tasks_into_queue(&self, tasks: VecDeque<Task<Data, Result>>) {
        let lock = self.thread_pool.lock();
        let mut pool_lock = lock.expect("Failed to acquire mutex in get_all_tasks_and_clear");
        let pool = pool_lock.deref_mut();
        for task in tasks {
            pool.tasks_queue.push_back(task);
        }
    }
}
#[cfg(test)]
mod test {
    use std::collections::VecDeque;
    use crate::utils::thread_pool::{Task, ThreadPoolWrap};

    fn is_prime(n: i128) -> bool{
        for i in 2..n {
            if i * i > n {
                break;
            }
            if n % i == 0 {
                return false;
            }
        }
        true
    }
    fn calc_nth_prime_slow(n: i32) -> i128 {
        let mut primes = 1;
        let mut cur_prime = 2;

        for i in 3.. {
            if primes == n {
                break
            }
            if is_prime(i) {
                cur_prime = i;
                primes += 1
            }
        }
        cur_prime
    }
    #[test]
    fn test_pool() {
        let twp = ThreadPoolWrap::<i32, (i32, i128)>::new(12);
        let mut tasks = VecDeque::<Task::<i32, (i32, i128)>>::new();
        for _ in 0..50 {
            tasks.push_back(Task::<i32, (i32, i128)> {
                task: |x| (x, calc_nth_prime_slow(x)),
                data: 50000,
            })
        }
        tasks.push_back(Task::<i32, (i32, i128)> {
            task: |x| (x, calc_nth_prime_slow(x)),
            data: 3,
        });
        twp.put_tasks_into_queue(tasks);
        twp.await_all_tasks();
        let res = twp.get_all_tasks_and_clear();
        for i in res {
            if i.0 == 3 {
                assert_eq!(i.1, 5);
            } else {
                assert_eq!(i.1, 611953);
            }
        }
    }
}

