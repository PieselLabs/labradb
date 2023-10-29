#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

pub mod parquet_reader;
pub mod printer;

pub trait Kernel<TResult> {
    fn next(&mut self) -> Option<TResult>;
}
