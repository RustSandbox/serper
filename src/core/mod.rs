/// Core module containing fundamental types and error handling
/// 
/// This module provides the foundational components used throughout the SDK.
pub mod error;
pub mod types;

pub use error::{SerperError, Result};
pub use types::{ApiKey, BaseUrl, Location, Pagination};