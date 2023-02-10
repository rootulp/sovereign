use crate::storage::{StorageKey, StorageValue};
use first_read_last_write_cache::{
    cache::{self, CacheLog},
    CacheValue,
};
use std::{cell::RefCell, rc::Rc};

pub(crate) trait GetValue {
    fn get_value(&self, key: StorageKey) -> Option<StorageValue>;
}

#[derive(Default, Clone)]
pub(crate) struct Cache {
    cache: Rc<RefCell<CacheLog>>,
}

impl Cache {
    pub(crate) fn get<G: GetValue>(
        &self,
        key: StorageKey,
        value_getter: &G,
    ) -> Option<StorageValue> {
        let cache_key = key.clone().as_cache_key();
        let cache_value = self.cache.borrow().get_value(&cache_key);

        match cache_value {
            cache::ExistsInCache::Yes(cache_value_exists) => {
                self.cache
                    .borrow_mut()
                    .add_read(cache_key, cache_value_exists.clone())
                    // It is ok to panic here, we must guarantee that the cache is consistent.
                    .unwrap_or_else(|e| panic!("Inconsistent read from the cache: {e:?}"));

                cache_value_exists.value.map(|value| StorageValue { value })
            }
            // TODO If the value does not exist in the cache, then fetch it from the JMT.
            cache::ExistsInCache::No => value_getter.get_value(key),
        }
    }

    pub(crate) fn set(&mut self, key: StorageKey, value: StorageValue) {
        let cache_key = key.as_cache_key();
        let cache_value = value.as_cache_value();
        self.cache.borrow_mut().add_write(cache_key, cache_value);
    }

    pub(crate) fn delete(&mut self, key: StorageKey) {
        let cache_key = key.as_cache_key();
        self.cache
            .borrow_mut()
            .add_write(cache_key, CacheValue::empty());
    }
}
