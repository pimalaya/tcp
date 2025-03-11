/// The TCP I/O request enum, emitted by flows and processed by
/// handlers.
///
/// This enum represents all the possible I/O requests that a TCP flow
/// can emit. I/O handlers should be able to handle all variants.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Io {
    /// I/O request that should be emitted by a flow needing bytes to
    /// be read in order to continue its progression.
    ///
    /// When receiving this variant, I/O handlers need to read a chunk
    /// of bytes using the [state buffer], then to tell the state [how
    /// many bytes] have been read for the current chunk.
    ///
    /// [state buffer]: super::State::get_read_buffer_mut
    /// [how many bytes]: super::State::set_read_bytes_count
    Read,
    /// I/O request that should be emitted by a flow needing bytes to
    /// be write in order to continue its progression.
    ///
    /// When receiving this variant, I/O handlers need to write a
    /// chunk of bytes using the [state buffer], then to tell the
    /// state [how many bytes] have been write for the current chunk.
    ///
    /// [state buffer]: super::State::get_write_buffer
    /// [how many bytes]: super::State::set_written_bytes_count
    Write,
}
