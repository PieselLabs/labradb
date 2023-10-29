#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

pub mod context;
pub mod execution_engine;
pub mod ir;

pub use context::*;
pub use execution_engine::*;
pub use ir::*;
pub use module::*;
