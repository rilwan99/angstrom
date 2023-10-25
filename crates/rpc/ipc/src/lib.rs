//! Guard IPC transport implementation
//!
//! ## Feature Flags
//!
//! - `client`: Enables JSON-RPC client support.

#![warn(missing_debug_implementations, missing_docs, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[cfg(unix)]
pub mod client;
pub mod server;

/// Json codec implementation
pub mod stream_codec;
