use crate::WorkQueue;

use super::{
    Builder, BuilderWithGlobalAndThreadContext, BuilderWithGlobalContext, BuilderWithThreadContext,
};

fn run_parallel<GlobalContext: Sync, ThreadContext, T: Send>(
    threads: usize,
    work_queue: &WorkQueue<T>,
    global_context: &GlobalContext,
    create_thread_context: &(impl Fn(&GlobalContext, usize) -> ThreadContext + Sync),
    worker: &(impl Fn(&GlobalContext, &mut ThreadContext, T) + Sync),
) {
    let threads = std::cmp::min(threads, work_queue.initial_len());

    std::thread::scope(|s| {
        for tid in 0..threads {
            s.spawn(move || {
                let mut thread_context = create_thread_context(&global_context, tid);

                while let Some(data) = work_queue.pop() {
                    worker(&global_context, &mut thread_context, data)
                }
            });
        }
    });
}

impl<T> Builder<T>
where
    T: Send,
{
    pub fn run(self, worker: impl Fn(T) + Sync) {
        run_parallel(
            self.data.threads,
            &self.data.work_queue,
            &(),
            &|_, _| (),
            &|_, _, work| worker(work),
        );
    }
}

impl<'a, T, GC> BuilderWithGlobalContext<'a, T, GC>
where
    T: Send,
    GC: Sync,
{
    pub fn run(self, worker: impl Fn(&GC, T) + Sync) {
        run_parallel(
            self.data.threads,
            &self.data.work_queue,
            self.global_context,
            &|_, _| (),
            &|gc, _, work| worker(gc, work),
        );
    }
}

impl<T, TC, CreateTC> BuilderWithThreadContext<T, CreateTC>
where
    T: Send,
    CreateTC: Fn(usize) -> TC + Sync,
{
    pub fn run(self, worker: impl Fn(&mut TC, T) + Sync) {
        let create_thread_context = &self.create_thread_context;

        run_parallel(
            self.data.threads,
            &self.data.work_queue,
            &(),
            &|_, tid| create_thread_context(tid),
            &|_, tc, work| worker(tc, work),
        );
    }
}

impl<'a, T, GC, TC, CreateTC> BuilderWithGlobalAndThreadContext<'a, T, GC, CreateTC>
where
    T: Send,
    GC: Sync,
    CreateTC: Fn(&GC, usize) -> TC + Sync,
{
    pub fn run(self, worker: impl Fn(&GC, &mut TC, T) + Sync) {
        run_parallel(
            self.data.threads,
            &self.data.work_queue,
            self.global_context,
            &self.create_thread_context,
            &worker,
        );
    }
}
