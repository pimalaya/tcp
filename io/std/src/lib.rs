#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

//! The [TCP flows] project is a set of libraries to manage TCP
//! streams in a I/O-agnostic way. It is highly recommended that you
//! read first about the project in order to understand `tcp-std`.
//!
//! This library exposes an I/O handler for that project, based on the
//! Rust standard library (sync).
//!
//! [TCP flows]: https://github.com/pimalaya/tcp

use std::{
    io::{self, Read, Result, Write},
    net::{SocketAddr, TcpStream},
};

use tcp_lib::{Io, State};
use tracing::{debug, instrument};

/// The standard, blocking TCP I/O handler.
///
/// This handler makes use of the standard, blocking
/// [`std::net::TcpStream`] to read from and write to TCP streams.
#[derive(Debug)]
pub struct Handler {
    stream: TcpStream,
}

impl Handler {
    /// Builds a new handler.
    ///
    /// This function does perform I/O, as it connects to the TCP
    /// stream matching the given hostname and port.
    #[instrument("tcp/std", skip_all)]
    pub fn new(host: impl AsRef<str>, port: u16) -> Result<Self> {
        let host = host.as_ref();
        debug!(?host, port, "connecting TCP stream");
        let stream = TcpStream::connect((host, port))?;
        debug!("connected");
        Ok(Self { stream })
    }

    /// Processes the [`Io`] request for the given flow's [`State`].
    #[instrument("tcp/std", skip_all)]
    pub fn handle(&mut self, io: Io, mut flow: impl AsMut<State>) -> Result<()> {
        match io {
            Io::Read => self.read(flow.as_mut()),
            Io::Write => self.write(flow.as_mut()),
        }
    }

    /// Processes the [`Io::Read`] request.
    ///
    /// This function reads synchronously a chunk of bytes from the
    /// inner TCP stream to its inner state read buffer, then set how
    /// many bytes have been read.
    #[instrument(skip_all)]
    pub fn read(&mut self, state: &mut State) -> Result<()> {
        let buffer = state.get_read_buffer_mut();
        let bytes_count = self.stream.read(buffer)?;
        state.set_read_bytes_count(bytes_count);
        Ok(())
    }

    /// Processes the [`Io::Write`] request.
    ///
    /// This function writes synchronously bytes to the inner TCP
    /// stream from its inner state write buffer, then set how many
    /// bytes have been written.
    #[instrument(skip_all)]
    pub fn write(&mut self, state: &mut State) -> Result<()> {
        let buffer = state.get_write_buffer();
        let bytes_count = self.stream.write(buffer)?;
        state.set_written_bytes_count(bytes_count);
        self.stream.flush()
    }
}

impl From<TcpStream> for Handler {
    fn from(stream: TcpStream) -> Self {
        Self { stream }
    }
}

impl TryFrom<SocketAddr> for Handler {
    type Error = io::Error;

    fn try_from(addr: SocketAddr) -> io::Result<Self> {
        let host = addr.ip();
        let port = addr.port();
        debug!(?host, port, "connecting TCP stream");
        let stream = TcpStream::connect(addr)?;
        debug!("connected");
        Ok(Self { stream })
    }
}

#[cfg(test)]
mod tests {
    use std::{
        io::{Read as _, Write as _},
        net::{TcpListener, TcpStream},
        thread,
    };

    use tcp_lib::flow::{Read, Write};

    use crate::Handler;

    fn new_tcp_stream_pair() -> (TcpStream, TcpStream) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let accept = thread::spawn(move || listener.accept().unwrap().0);
        let client = TcpStream::connect(addr).unwrap();
        let server = accept.join().unwrap();
        (client, server)
    }

    #[test]
    fn read() {
        let (mut client, server) = new_tcp_stream_pair();
        let mut handler = Handler::from(server);

        let written_bytes = b"data".to_vec();
        client.write(&written_bytes).unwrap();

        let mut flow = Read::new();

        let read_bytes: Vec<u8> = loop {
            match flow.next() {
                Ok(bytes) => {
                    break bytes.to_vec();
                }
                Err(io) => {
                    handler.handle(io, &mut flow).unwrap();
                }
            }
        };

        assert_eq!(written_bytes, read_bytes)
    }

    #[test]
    fn read_chunks() {
        let (mut client, server) = new_tcp_stream_pair();
        let mut handler = Handler::from(server);

        let written_bytes = b"big data ended by dollar$".to_vec();
        client.write(&written_bytes).unwrap();

        let mut flow = Read::with_capacity(3);
        let mut read_bytes = Vec::new();

        loop {
            let bytes = match flow.next() {
                Ok(bytes) => bytes.to_vec(),
                Err(io) => {
                    handler.handle(io, &mut flow).unwrap();
                    continue;
                }
            };

            println!("bytes: {read_bytes:?}");

            read_bytes.extend(bytes);

            if let Some(b'$') = read_bytes.last() {
                break;
            }
        }

        assert_eq!(written_bytes, read_bytes);
    }

    #[test]
    fn write() {
        let (mut client, server) = new_tcp_stream_pair();
        let mut handler = Handler::from(server);

        let mut flow = Write::new();
        flow.enqueue_bytes(b"data".to_vec());

        let written_bytes: Vec<u8> = loop {
            match flow.next() {
                Ok(bytes) => {
                    break bytes.to_vec();
                }
                Err(io) => {
                    handler.handle(io, &mut flow).unwrap();
                }
            }
        };

        let mut read_bytes = [0; 4];
        client.read(&mut read_bytes).unwrap();

        assert_eq!(written_bytes, read_bytes)
    }
}
