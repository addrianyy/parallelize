use crate::WorkQueue;

mod context;
mod run;

struct BuilderData<T> {
    threads: usize,
    work_queue: WorkQueue<T>,
}

impl<T> BuilderData<T> {
    fn new(work_queue: WorkQueue<T>) -> Self {
        let threads = std::thread::available_parallelism()
            .expect("failed to get available threads")
            .get();

        Self {
            threads,
            work_queue,
        }
    }

    fn with_threads(self, threads: usize) -> Self {
        assert!(threads > 0, "thread count must be higher than 0");
        Self {
            threads,
            work_queue: self.work_queue,
        }
    }
}

pub struct Builder<T> {
    data: BuilderData<T>,
}

impl<T: Send> Builder<T> {
    pub(super) fn new(data: impl Iterator<Item = T>) -> Builder<T> {
        Builder {
            data: BuilderData::new(WorkQueue::new(data)),
        }
    }
}

pub struct BuilderWithGlobalContext<'a, T, GC> {
    data: BuilderData<T>,
    global_context: &'a GC,
}

pub struct BuilderWithThreadContext<T, CreateTC> {
    data: BuilderData<T>,
    create_thread_context: CreateTC,
}

pub struct BuilderWithGlobalAndThreadContext<'a, T, GC, CreateTC> {
    data: BuilderData<T>,
    global_context: &'a GC,
    create_thread_context: CreateTC,
}

impl<T> Builder<T> {
    pub fn with_threads(self, threads: usize) -> Self {
        Self {
            data: self.data.with_threads(threads),
            ..self
        }
    }
}

impl<'a, T, GC> BuilderWithGlobalContext<'a, T, GC> {
    pub fn with_threads(self, threads: usize) -> Self {
        Self {
            data: self.data.with_threads(threads),
            ..self
        }
    }
}

impl<T, CreateTC> BuilderWithThreadContext<T, CreateTC> {
    pub fn with_threads(self, threads: usize) -> Self {
        Self {
            data: self.data.with_threads(threads),
            ..self
        }
    }
}

impl<'a, T, GC, CreateTC> BuilderWithGlobalAndThreadContext<'a, T, GC, CreateTC> {
    pub fn with_threads(self, threads: usize) -> Self {
        Self {
            data: self.data.with_threads(threads),
            ..self
        }
    }
}
