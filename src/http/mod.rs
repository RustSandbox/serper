pub mod client;
/// HTTP module containing transport and client functionality
///
/// This module provides HTTP transport layer abstractions and high-level
/// client functionality for interacting with the Serper API.
pub mod transport;

pub use client::{SerperHttpClient, SerperHttpClientBuilder};
pub use transport::{HttpTransport, HttpTransportBuilder, TransportConfig};
