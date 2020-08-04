use std::collections::HashMap;
use threadpool::ThreadPool;

pub struct ThreadUtils {
    pools: HashMap<String, ThreadPool>
}

static mut el: *mut ThreadUtils = 0 as *mut _;
const DEFAULT_THREADS: usize = 1;

impl ThreadUtils {
    pub fn instance() -> &'static mut ThreadUtils {
        unsafe {
            if el == 0 as *mut _ {
                let config = ThreadUtils{pools: HashMap:: new()};
                el = Box::into_raw(Box::new(config));
            }
            &mut *el
        }
    }

    pub fn create_pool(&mut self, name: String, threads: usize) {
        let pool = ThreadPool::with_name(name.clone(), threads);
        self.pools.insert(name, pool);
    }

    pub fn get_pool(&mut self, name: &String) -> &mut ThreadPool {
        if !self.pools.contains_key(name) {
            self.pools.insert(name.clone(), ThreadPool::with_name(name.clone(), DEFAULT_THREADS));
        }

        self.pools.get_mut(name).unwrap()
    }
}