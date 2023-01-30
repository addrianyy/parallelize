use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    sync::atomic::{AtomicUsize, Ordering},
};

pub struct WorkQueue<T> {
    data: Box<[UnsafeCell<Option<T>>]>,
    next_free_index: AtomicUsize,
    _phantom_data: PhantomData<T>,
}

impl<T> WorkQueue<T> {
    pub fn new(items: impl Iterator<Item = T>) -> Self {
        Self {
            data: items.map(|x| UnsafeCell::new(Some(x))).collect(),
            next_free_index: AtomicUsize::new(0),
            _phantom_data: PhantomData,
        }
    }

    pub fn pop(&self) -> Option<T> {
        let index = self.next_free_index.fetch_add(1, Ordering::Relaxed);
        assert!(index < usize::MAX / 2, "index reached max possible value");

        if index < self.data.len() {
            let data = unsafe { (*self.data[index].get()).take() };
            debug_assert!(data.is_some(), "data shouldn't be empty at this point");
            data
        } else {
            None
        }
    }

    pub fn initial_len(&self) -> usize {
        self.data.len()
    }
}

unsafe impl<T: Send> Send for WorkQueue<T> {}
unsafe impl<T: Send> Sync for WorkQueue<T> {}
