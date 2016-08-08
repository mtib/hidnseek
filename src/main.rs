pub mod client;
pub mod server;
pub mod code;
pub mod field;

use client::Client;
use server::Server;

use std::thread;
use std::time;
use std::env;

// TODO Usage string
// TODO man page?

fn main() {
    let mut argiter = env::args();
    match argiter.nth(1).unwrap_or_default().as_str() {
        "s" => {
            // just start the server
            let mut s = Server::new();
            s.start();
        }
        "c" => {
            // start the client
            let (mut client, _) = match argiter.next() {
                // connect to a server, no need for local server
                Some(addr) => (Client::new(String::from(addr)), None),
                // do not connect to an online server, create local one
                None => {
                    // connect client to localhost:3377 (server)
                    let s = (Client::new("127.0.0.1".to_owned()),
                    // start local server
                    Some(thread::spawn(move || {
                        let mut s = Server::new();
                        // make server output more obvious:
                        // s.output_delim("[LOCAL_SERVER] "," [LOCAL_SERVER]");
                        s.disable_output();
                        s.start();
                    })));
                    // the server has to be up and running before the client
                    thread::sleep(time::Duration::new(1, 0));
                    s
                }
            };
            // set up the client
            client.connect();
            // give the user control
            client.control_loop();
        }
        o => println!("use s or c, not: {:?}", o),
    }
}
