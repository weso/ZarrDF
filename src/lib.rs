mod config;
mod dictionary;
pub mod error;
mod index;
pub mod query;
mod rdf_format;
pub mod storage;
mod utils;

pub(crate) type Triple = [u64; 3];
