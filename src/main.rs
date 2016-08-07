pub mod client;
pub mod server;

use client::Client;
use server::Server;

use std::env;

fn main() {
    match env::args().nth(1).unwrap().as_str() {
        "s" => {
            let mut s = Server::new();
            s.start();
        },
        "c" => {
            let mut c = Client::new();
            c.connect();
            c.control_loop();
        },
        o => println!("use s or c, not: {:?}", o),
    }
}
