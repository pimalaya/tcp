use tracing::instrument;

use crate::{Io, State};

/// Flow for reading a chunk of bytes from a TCP stream.
///
/// This flow should be used when you need to read a chunk of bytes
/// from a TCP stream. The chunk size depends on the I/O state's
/// internal read buffer capacity, which can be adjusted with
/// [`Read::with_capacity`].
#[derive(Debug, Default)]
pub struct Read {
    state: State,
}

impl Read {
    /// Creates a new read flow using defaults.
    #[instrument(skip_all)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new read flow using the given read buffer capacity.
    #[instrument(skip_all)]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            state: State::new(capacity),
        }
    }

    #[instrument(skip_all)]
    pub fn next(&mut self) -> Result<&[u8], Io> {
        match self.state.bytes_count.take() {
            Some(n) => Ok(self.state.get_read_bytes(n)),
            None => Err(Io::Read),
        }
    }
}

impl AsMut<State> for Read {
    fn as_mut(&mut self) -> &mut State {
        &mut self.state
    }
}
