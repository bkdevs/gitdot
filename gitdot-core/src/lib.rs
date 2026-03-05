pub mod client;
pub mod dto;
pub mod error;
pub mod model;
pub mod repository;
pub mod service;

pub(crate) mod util;
pub(crate) use gitdot_core_derive::instrument_all;
