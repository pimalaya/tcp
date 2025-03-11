#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

//! The [TCP flows] project is a set of libraries to manage TCP
//! streams in a I/O-agnostic way. It is highly recommended that you
//! read first about the project in order to understand `tcp-lib`.
//!
//! This library gathers all the I/O-free part of the project,
//! including:
//!
//! - The [`Io`] request enum
//! - The I/O-free [`flow`]s
//! - The shared [`State`] between flows and handlers
//!
//! [TCP flows]: https://github.com/pimalaya/tcp

pub mod flow;
mod io;
mod state;

#[doc(inline)]
pub use self::{io::Io, state::State};
