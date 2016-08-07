#![allow(dead_code)]
use std::net::UdpSocket;

const ADDR: &'static str = r"127.0.0.1";
pub const PORT: u16 = 3377;
pub const RECV_SIZE: usize = 256;
pub const DEFAULT_MAX_PLAYERS: usize = 8;

pub struct Server {
    max_players: usize,
    cur_players: usize,
    name: String,
    width: usize,
    height: usize
}

/*
Server Codes:
200 login
400 logout
800 chat
*/

impl Server {
    pub fn new() -> Self{
        Server{
            max_players: DEFAULT_MAX_PLAYERS,
            cur_players: 0,
            name: String::from("Default Server Name"),
            width: 4,
            height: 4
        }
    }
    pub fn start(&mut self) {
        let socket = UdpSocket::bind((ADDR, PORT))
            .expect("Cound not create server socket!");

        loop {
            // read from the socket
            let mut buf = [0u8; RECV_SIZE];
            let (_, src) = socket.recv_from(&mut buf)
                .expect("Could not speak with outside world!");
            // send a reply to the socket we received data from
            let code = String::from_utf8_lossy(&buf[..3]);
            let content = String::from_utf8_lossy(&buf[4..]);
            println!("[{}]: {}", code, content);
            //let buf = buf[..];
            match socket.send_to(&buf, &src){
                Ok(_) => (),
                Err(e) => println!("{:?}", e)
            }
        }
    }
}
