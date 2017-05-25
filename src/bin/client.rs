extern crate clap;
extern crate futures;
extern crate futures_io;
extern crate futures_mio;

use clap::{Arg, App};
use std::net::{SocketAddr, Shutdown};

use futures::Future;
use futures_mio::Loop;

fn main() {
    let matches = App::new("Rust Echo Client")
        .version("0.1.0")
        .author("Sean Chen <seanchen11235@gmail.com>")
        .about("A simple Rust echo client")
        .arg(Arg::with_name("server")
             .short("s")
             .long("server")
             .takes_value(true)
             .help("The name of the server to connect to. Defaults to localhost."))
        .arg(Arg::with_name("port")
             .short("p")
             .long("port")
             .takes_value(true)
             .help("The port number to connect to. Defaults to 8080."))
        .arg(Arg::with_name("message")
             .short("m")
             .long("message")
             .takes_value(true)
             .help("The message to send to the server. Defaults to 'Hello World!'."))
        .get_matches();

    let server = matches.value_of("server").unwrap_or("localhost");
    let port = matches.value_of("port").unwrap_or("8080");
    let message = matches.value_of("message").unwrap_or("Hello World!");

    let mut loop = Loop::new().unwrap();
    let addr = &server + ":" + &port;

    let addr = addr.parse::<SocketAddr>().unwrap();
    let socket = loop.handle().tcp_connect(&addr);

    let request = socket.and_then(|socket| {
        futures_io.write_all(socket, message)
    });

    let response = request.and_then(|(socket, _)| {
        socket.shutdown(std::net::Shutdown::Write).expect("Could not shut down");
        futures_io::read_to_end(socket, Vec::new())
    });

    let data = loop.run(response).unwrap();
    println!("{}", String::from_utf8_lossy(&data));
}
