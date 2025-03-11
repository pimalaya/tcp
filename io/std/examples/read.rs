use std::{
    io::{stderr, Write as _},
    net::{SocketAddr, TcpListener},
    thread::{self, JoinHandle},
};

use tcp_lib::flow::Read;
use tcp_std::Handler;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn main() {
    info!("init logger and TCP server");
    init_logger();
    let (addr, server) = init_server();

    info!("init TCP I/O handler");
    let mut tcp = Handler::try_from(addr).unwrap();

    server.join().unwrap();

    info!("read bytes on TCP stream using Read flow");
    let mut flow = Read::new();

    let output = loop {
        match flow.next() {
            Ok(output) => break output.to_vec(),
            Err(io) => tcp.handle(io, &mut flow).unwrap(),
        }
    };

    info!("read bytes: {:?}", String::from_utf8_lossy(&output));
}

fn init_logger() {
    let layer = fmt::layer().with_writer(stderr);
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(layer)
        .with(filter)
        .init();
}

fn init_server() -> (SocketAddr, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let host = addr.ip();
    let port = addr.port();

    info!(?host, port, "spawn TCP server");

    let handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream.write(b"Lorem ipsum dolor sit amet.").unwrap();
    });

    (addr, handle)
}
