#[path = "generated/buffa/mod.rs"]
pub mod proto;
#[path = "generated/connect/mod.rs"]
pub mod connect;

pub mod core;
pub mod db;
pub mod infra;
pub mod event_bus;
pub mod llm_client;
pub mod message_bus;
pub mod process_manager;
pub mod tools;
pub mod service;
pub mod api;
pub mod config;
pub mod file_time;
pub mod external_tools;
pub mod prompts;

pub use core::*;
pub use infra::{AppError, ErrorCode, Errors};