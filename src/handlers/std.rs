//! Module dedicated to the standard, blocking stream I/O handler.

use std::io::{Read, Result, Write};

use crate::{Io, State};

/// The standard, blocking stream I/O handler.
pub struct Handler;

impl Handler {
    /// Processes the [`Io`] request for the given flow, onto the
    /// given stream.
    pub fn handle(stream: impl StreamExt, mut flow: impl AsMut<State>, io: Io) -> Result<usize> {
        match io {
            Io::Read => Self::read(stream, flow.as_mut()),
            Io::Write => Self::write(stream, flow.as_mut()),
        }
    }

    /// Processes the [`Io::Read`] request for the given flow's
    /// [`State`], onto the given stream.
    ///
    /// This function reads synchronously a chunk of bytes from the
    /// given stream to the given state's read buffer, then set how
    /// many bytes have been read.
    pub fn read(mut stream: impl Read, state: &mut State) -> Result<usize> {
        let buffer = state.get_read_buffer_mut();
        let bytes_count = stream.read(buffer)?;
        state.set_read_bytes_count(bytes_count);
        Ok(bytes_count)
    }

    /// Processes the [`Io::Write`] request for the given flow's
    /// [`State`], onto the given stream.
    ///
    /// This function writes synchronously bytes to the given stream
    /// from the given state's write buffer, then set how many bytes
    /// have been written.
    pub fn write(mut stream: impl Write, state: &mut State) -> Result<usize> {
        let buffer = state.get_write_buffer();
        let bytes_count = stream.write(buffer)?;
        state.set_written_bytes_count(bytes_count);
        Ok(bytes_count)
    }
}

/// Stream trait extension.
///
/// This trait is just a helper for structures that implement both
/// [`Read`] and [`Write`].
pub trait StreamExt: Read + Write {}

impl<T: Read + Write> StreamExt for T {}
