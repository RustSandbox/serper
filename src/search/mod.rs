/// Search module containing query construction and response handling
///
/// This module provides comprehensive functionality for building search queries,
/// handling responses, and orchestrating search operations.
pub mod query;
pub mod response;
pub mod service;

pub use query::{SearchQuery, SearchQueryBuilder};
pub use response::{
    AnswerBox, KnowledgeGraph, NewsResult, OrganicResult, RelatedQuestion, ResponseParser,
    SearchMetadata, SearchResponse, ShoppingResult,
};
pub use service::SearchService;
