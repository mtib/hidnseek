mod client;
mod server;

use std::env;

fn main() {
    match env::args().nth(1).unwrap().as_str() {
        "s" => {
            let mut s = server::new();
            s.start();
        },
        "c" => {
            let mut c = client::new();
            c.connect();
        },
        o => println!("use s or c, not: {:?}", o),
    }
}
