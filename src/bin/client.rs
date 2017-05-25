extern crate clap;

use clap::{Arg, App};

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

    println!("The specified server is: {}", server);
    println!("The specified port is: {}", port);
    println!("The specified message is: {}", message);
}
