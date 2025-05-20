//! Module dedicated to the [`Read`] I/O-free coroutine.

use log::debug;

use crate::{Io, Output};

/// I/O-free coroutine for reading bytes into a buffer.
#[derive(Debug)]
pub struct Read {
    buffer: Option<Vec<u8>>,
}

impl Read {
    /// Creates a new coroutine from the given buffer mutable
    /// reference.
    pub fn new(buffer: Vec<u8>) -> Self {
        let capacity = buffer.capacity();
        debug!("prepare buffer of {capacity} capacity to be read");
        let buffer = Some(buffer);
        Self { buffer }
    }

    pub fn replace(&mut self, buffer: Vec<u8>) {
        *self = Self::new(buffer);
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, input: Option<Io>) -> Result<Output, Io> {
        let Some(input) = input else {
            return Err(match self.buffer.take() {
                Some(buffer) => Io::Read(Err(buffer)),
                None => Io::UnavailableInput,
            });
        };

        let Io::Read(output) = input else {
            return Err(Io::UnexpectedInput(Box::new(input)));
        };

        match output {
            Ok(output) => {
                let n = output.bytes_count;
                let capacity = output.buffer.capacity();
                debug!("resume after {n}/{capacity} bytes read");
                Ok(output)
            }
            Err(io) => {
                debug!("break: need I/O to read bytes");
                Err(Io::Read(Err(io)))
            }
        }
    }
}

impl Default for Read {
    fn default() -> Self {
        Self::new(vec![0; 1024])
    }
}
