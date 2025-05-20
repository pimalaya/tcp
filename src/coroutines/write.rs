//! Module dedicated to the [`Write`] I/O-free coroutine.

use log::debug;

use crate::{Io, Output};

/// I/O-free coroutine for writing bytes into a stream.
#[derive(Debug, Default)]
pub struct Write {
    bytes: Option<Vec<u8>>,
}

impl Write {
    /// Creates a new coroutine from the given buffer reference.
    pub fn new(bytes: impl IntoIterator<Item = u8>) -> Self {
        let bytes: Vec<u8> = bytes.into_iter().collect();
        let n = bytes.len();
        debug!("prepare {n} bytes to be written");
        let bytes = Some(bytes);
        Self { bytes }
    }

    pub fn replace(&mut self, bytes: impl IntoIterator<Item = u8>) {
        *self = Self::new(bytes);
    }

    pub fn extend(&mut self, more_bytes: impl IntoIterator<Item = u8>) {
        match &mut self.bytes {
            Some(bytes) => {
                let prev_len = bytes.len();
                bytes.extend(more_bytes);
                let next_len = bytes.len();
                let n = next_len - prev_len;
                debug!("prepare {prev_len}+{n} additional bytes to be written");
            }
            None => self.replace(more_bytes),
        }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, input: Option<Io>) -> Result<Output, Io> {
        let Some(input) = input else {
            return Err(match self.bytes.take() {
                Some(bytes) => Io::Write(Err(bytes)),
                None => Io::UnavailableInput,
            });
        };

        let Io::Write(output) = input else {
            return Err(Io::UnexpectedInput(Box::new(input)));
        };

        match output {
            Ok(output) => {
                let n = output.bytes_count;
                debug!("resume after {n} bytes written");
                Ok(output)
            }
            Err(io) => {
                debug!("break: need I/O to write bytes");
                Err(Io::Write(Err(io)))
            }
        }
    }
}
