#![cfg(feature = "std")]

use std::{
    env,
    io::{stdin, stdout, Write as _},
    net::TcpStream,
    sync::Arc,
};

use io_stream::{
    coroutines::{Read, Write},
    runtimes::std::handle,
};
use memchr::memmem;
use rustls::{ClientConfig, ClientConnection, StreamOwned};
use rustls_platform_verifier::ConfigVerifierExt;
use url::Url;

fn main() {
    env_logger::init();

    let url: Url = match env::var("URL") {
        Ok(url) => url.parse().unwrap(),
        Err(_) => read_line("URL?").parse().unwrap(),
    };

    let mut stream = connect(&url);

    let request = format!("GET {} HTTP/1.0\r\n\r\n", url.path());

    let mut arg = None;
    let mut write = Write::new(request.into_bytes());

    while let Err(io) = write.resume(arg) {
        arg = Some(handle(&mut stream, io).unwrap());
    }

    let mut response = Vec::new();

    loop {
        let mut arg = None;
        let mut read = Read::new();

        let output = loop {
            match read.resume(arg) {
                Ok(output) => break output,
                Err(io) => arg = Some(handle(&mut stream, io).unwrap()),
            }
        };

        let bytes = output.bytes();

        match memmem::find(bytes, &[b'\r', b'\n', b'\r', b'\n']) {
            None => {
                response.extend(bytes);
                continue;
            }
            Some(n) => {
                response.extend(&bytes[..n]);
                break;
            }
        }
    }

    println!("----------------");
    println!("{}", String::from_utf8_lossy(&response));
    println!("----------------");
}

fn read_line(prompt: &str) -> String {
    print!("{prompt} ");
    stdout().flush().unwrap();

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim().to_owned()
}

trait StreamExt: std::io::Read + std::io::Write {}
impl<T: std::io::Read + std::io::Write> StreamExt for T {}

fn connect(url: &Url) -> Box<dyn StreamExt> {
    let domain = url.domain().unwrap();

    if url.scheme().eq_ignore_ascii_case("https") {
        let config = ClientConfig::with_platform_verifier();
        let server_name = domain.to_string().try_into().unwrap();
        let conn = ClientConnection::new(Arc::new(config), server_name).unwrap();
        let tcp = TcpStream::connect((domain.to_string(), 443)).unwrap();
        let tls = StreamOwned::new(conn, tcp);
        Box::new(tls)
    } else {
        let tcp = TcpStream::connect((domain.to_string(), 80)).unwrap();
        Box::new(tcp)
    }
}
