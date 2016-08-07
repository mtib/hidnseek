pub mod client;
pub mod server;
pub mod code;

use client::Client;
use server::Server;

use std::thread;
use std::time;
use std::env;

fn main() {
    let mut argiter = env::args();
    match argiter.nth(1).unwrap().as_str() {
        "s" => {
            let mut s = Server::new();
            s.start();
        },
        "c" => {
            let (mut client, _) = match argiter.next() {
                Some(addr) => {
                    (Client::new(String::from(addr)), None)
                }
                None => {
                    (
                        Client::new("localhost".to_owned()),
                        Some(thread::spawn(move || {
                            let mut s = Server::new();
                            // s.output_delim("[LOCAL_SERVER] ", " [LOCAL_SERVER]");
                            s.disable_output();
                            s.start();
                        }))
                    )
                }
            };
            thread::sleep(time::Duration::new(1,0));
            client.connect();
            client.control_loop();
        },
        o => println!("use s or c, not: {:?}", o),
    }
}
