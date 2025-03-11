use tracing::instrument;

use crate::{Io, State};

/// Flow for writeing a chunk of bytes from a TCP stream.
///
/// This flow should be used when you need to write bytes into a TCP
/// stream.
#[derive(Debug, Default)]
pub struct Write {
    state: State,
}

impl Write {
    /// Creates a new write flow using the given write buffer capacity.
    #[instrument(skip_all)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enqueue_bytes(&mut self, bytes: impl IntoIterator<Item = u8>) {
        self.state.enqueue_bytes(bytes);
    }

    #[instrument(skip_all)]
    pub fn next(&mut self) -> Result<&[u8], Io> {
        match self.state.bytes_count.take() {
            Some(n) => Ok(&self.state.write_buffer[..n]),
            None => Err(Io::Write),
        }
    }
}

impl AsMut<State> for Write {
    fn as_mut(&mut self) -> &mut State {
        &mut self.state
    }
}
