pub mod parquet_reader;

use std::future::Future;

pub trait Kernel<TResult> {
    fn next(&mut self) -> Option<TResult>;
}
