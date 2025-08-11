/// Search module containing query construction and response handling
/// 
/// This module provides comprehensive functionality for building search queries,
/// handling responses, and orchestrating search operations.
pub mod query;
pub mod response;
pub mod service;

pub use query::{SearchQuery, SearchQueryBuilder};
pub use response::{
    SearchResponse, SearchMetadata, OrganicResult, AnswerBox, 
    KnowledgeGraph, RelatedQuestion, ShoppingResult, NewsResult,
    ResponseParser
};
pub use service::SearchService;