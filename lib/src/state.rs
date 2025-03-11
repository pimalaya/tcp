use tracing::{debug, instrument};

/// The TCP I/O state, owned by flows and updated by handlers.
///
/// This struct represents the I/O state used by I/O handlers to take
/// input and set output. It is usually owned by flows themselves, and
/// serve as communication bridge between flows and I/O handlers.
#[derive(Debug)]
pub struct State {
    pub(crate) read_buffer: Vec<u8>,
    pub(crate) write_buffer: Vec<u8>,
    pub(crate) bytes_count: Option<usize>,
}

impl State {
    pub const DEFAULT_READ_CAPACITY: usize = 512;

    /// Builds a new I/O state with the given reading capacity.
    ///
    /// See [`State::default`] for building a state with a default
    /// read capacity of [`State::DEFAULT_READ_CAPACITY`].
    #[instrument(skip_all)]
    pub fn new(capacity: usize) -> Self {
        Self {
            read_buffer: vec![0; capacity],
            write_buffer: Default::default(),
            bytes_count: Default::default(),
        }
    }

    /// Gets a mutable reference to the read buffer.
    pub fn get_read_buffer_mut(&mut self) -> &mut [u8] {
        &mut self.read_buffer
    }

    /// Gets read bytes matching the given count.
    pub fn get_read_bytes(&self, bytes_count: usize) -> &[u8] {
        &self.read_buffer[..bytes_count]
    }

    /// Sets how many bytes have been read.
    pub fn set_read_bytes_count(&mut self, bytes_count: usize) {
        debug!("read {bytes_count}/{} bytes", self.read_buffer.len());
        self.bytes_count.replace(bytes_count);
    }

    /// Enqueues bytes to the write buffer.
    pub fn enqueue_bytes(&mut self, bytes: impl IntoIterator<Item = u8>) {
        self.write_buffer.extend(bytes);
    }

    /// Gets a reference to the write buffer.
    pub fn get_write_buffer(&self) -> &[u8] {
        &self.write_buffer
    }

    /// Sets how many bytes have been written.
    pub fn set_written_bytes_count(&mut self, bytes_count: usize) {
        debug!("wrote {bytes_count} bytes");
        self.bytes_count.replace(bytes_count);
    }

    /// Gets how many bytes have been read/written, then resets it.
    pub fn take_bytes_count(&mut self) -> Option<usize> {
        self.bytes_count.take()
    }
}

impl Default for State {
    #[instrument(skip_all)]
    fn default() -> Self {
        Self::new(Self::DEFAULT_READ_CAPACITY)
    }
}
