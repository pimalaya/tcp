//! Module dedicated to the [`Write`] I/O-free coroutine.

use log::debug;

use crate::{Io, Output};

/// I/O-free coroutine for writing bytes into a stream.
#[derive(Debug, Default)]
pub struct Write {
    bytes: Option<Vec<u8>>,
}

impl Write {
    /// Creates a new coroutine to write the given bytes.
    pub fn new(bytes: impl IntoIterator<Item = u8>) -> Self {
        let bytes: Vec<u8> = bytes.into_iter().collect();
        let n = bytes.len();
        debug!("prepare {n} bytes to be written");
        let bytes = Some(bytes);
        Self { bytes }
    }

    /// Replaces the inner bytes with the given one.
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

    /// Makes the write progress.
    pub fn resume(&mut self, arg: Option<Io>) -> Result<Output, Io> {
        let Some(arg) = arg else {
            let Some(bytes) = self.bytes.take() else {
                return Err(Io::err("write bytes not ready"));
            };

            debug!("break: need I/O to write bytes");
            return Err(Io::Write(Err(bytes)));
        };

        debug!("resume after writting bytes");

        let Io::Write(Ok(output)) = arg else {
            let msg = format!("expected write output, got {arg:?}");
            return Err(Io::err(msg));
        };

        let n = output.bytes_count;
        debug!("wrote {n} bytes");

        Ok(output)
    }
}
