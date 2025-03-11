use std::{
    env,
    io::{stdin, stdout, BufRead, Write},
    net::TcpStream,
};

use stream_flows::{handlers::std::Handler, State};

fn main() {
    env_logger::init();

    let host = match env::var("HOST") {
        Ok(host) => host,
        Err(_) => read_line("TCP server hostname?"),
    };

    let port: u16 = match env::var("PORT") {
        Ok(port) => port.parse().unwrap(),
        Err(_) => read_line("TCP server port?").parse().unwrap(),
    };

    print!("connecting to {host}:{port}…");
    stdout().flush().unwrap();
    let mut tcp = TcpStream::connect((host.as_str(), port)).unwrap();
    println!(" [OK]");
    println!();

    let mut state = State::new(2048);

    println!("greeting:");
    let n = Handler::read(&mut tcp, &mut state).unwrap();
    for line in state.get_read_bytes(n).lines() {
        println!("S: {}", line.unwrap());
    }

    loop {
        println!();

        let input = read_line("C:");
        state.enqueue_bytes(input.as_bytes());
        state.enqueue_bytes(b"\r\n");
        Handler::write(&mut tcp, &mut state).unwrap();

        let n = Handler::read(&mut tcp, &mut state).unwrap();
        for line in state.get_read_bytes(n).lines() {
            println!("S: {}", line.unwrap());
        }
    }
}

fn read_line(prompt: &str) -> String {
    print!("{prompt} ");
    stdout().flush().unwrap();

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim().to_owned()
}
