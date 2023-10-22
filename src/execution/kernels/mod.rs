pub mod parquet_reader;



pub trait Kernel<TResult> {
    fn next(&mut self) -> Option<TResult>;
}
