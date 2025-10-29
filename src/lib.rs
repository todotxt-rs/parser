#![warn(warnings)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod date;
pub mod parser;
pub mod task;

#[cfg(feature = "config")]
mod color;
#[cfg(feature = "config")]
mod config;
mod errors;
mod priority;

#[cfg(feature = "config")]
pub use config::Config;
pub use date::Date;
pub use errors::*;
pub use priority::Priority;
pub use task::Task;

#[cfg(feature = "config")]
use color::Color;
