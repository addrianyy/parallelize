mod builders;
mod work_queue;

pub use work_queue::WorkQueue;

pub fn parallelize<T: Send>(data: impl Iterator<Item = T>) -> builders::Builder<T> {
    builders::Builder::new(data)
}
