pub mod parquet_reader;
pub mod print;

pub trait Kernel<TResult> {
    fn next(&mut self) -> Option<TResult>;
}
