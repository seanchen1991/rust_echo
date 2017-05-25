extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Rust Echo Server")
        .version("0.1.0")
        .author("Sean Chen <seanchen11235@gmail.com>")
        .about("A simple Rust echo server")
        .arg(Arg::with_name("port")
             .short("p")
             .long("port")
             .takes_value(true)
             .help("The port number to connect to. Defaults to 8080."))
        .arg(Arg::with_name("maxnpending")
             .short("n")
             .long("maxnpending")
             .takes_value(true)
             .help("The max number of pending connections. Defaults to 10."))
        .get_matches();

    let port = matches.value_of("port").unwrap_or("8080");
    let maxnpending = matches.value_of("maxnpending").unwrap_or("10");

    println!("The specified port is: {}", port);
    println!("The maximum number of pending connections is: {}", maxnpending);
}
