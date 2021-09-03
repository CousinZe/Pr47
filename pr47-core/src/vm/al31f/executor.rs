//! # `executor.rs`: core executor of AL31F
//!
//! ## ⚠️⚠️⚠️ Develop stage note ⚠️⚠️⚠
//! By this time the developers don't know what's the correct abstraction. This `executor` module
//! is temporary, maybe just here for testing. Project structure may change a lot in further days.

pub mod sync_executor;
pub use sync_executor::*;

pub mod async_executor;
pub use async_executor::*;
