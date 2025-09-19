//! Ecosystem integration system
//!
//! This module provides integration with various web frameworks,
//! build tools, and deployment platforms.

pub mod leptos_integration;
pub mod framework_adapters;
pub mod build_tools;
pub mod deployment;
pub mod ssr_support;
pub mod unified_api;

// Re-export main types for convenience
pub use leptos_integration::*;
pub use framework_adapters::*;
pub use build_tools::*;
pub use deployment::*;
pub use ssr_support::*;
pub use unified_api::*;
