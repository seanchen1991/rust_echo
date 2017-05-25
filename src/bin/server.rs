extern crate futures;
extern crate futures_io;
extern crate futures_mio;

use std::env;
use std::net::SocketAddr;

use futures::Future;
use futures::stream::Stream;
use futures_io::{copy, TaskIo};

fn main() { 
   let addr = env::args().nth(1).unwrap_or("localhost:8080".to_string());
   let addr = addr.parse::<SocketAddr>().unwrap();

   let mut loop = futures_mio::Loop::new().unwrap();

   let server = loop.handle().tcp_listen(&addr);

   let done = server.and_then(move |socket| {
       println!("Listening on: {}", addr);

       socket.incoming().for_each(|(socket, addr)| {
           let io = TaskIo::new(socket);
           let pair = io.map(|io| io.split());
           let amount = pair.and_then(|(reader, writer)| {
               copy(reader, writer)
           });
           amount.map(move |amt| {
               println!("Wrote {} bytes to {}", amt, addr)
           }).forget();

           Ok(())
       })
   });

   loop.run(done).unwrap();
}
