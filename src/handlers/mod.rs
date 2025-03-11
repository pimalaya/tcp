//! Collection of stream I/O handlers.
//!
//! The I/O handler contains all the I/O logic. It takes input from
//! the flow, processes the requested I/O, then puts the output back
//! inside the flow.
//!
//! If you miss a handler matching your requirements, you can easily
//! implement your own by taking example on the existing ones. PRs are
//! welcomed!

#[cfg(feature = "std")]
pub mod std;
