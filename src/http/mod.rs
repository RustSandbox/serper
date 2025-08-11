/// HTTP module containing transport and client functionality
/// 
/// This module provides HTTP transport layer abstractions and high-level
/// client functionality for interacting with the Serper API.
pub mod transport;
pub mod client;

pub use transport::{HttpTransport, TransportConfig, HttpTransportBuilder};
pub use client::{SerperHttpClient, SerperHttpClientBuilder};