#![allow(dead_code)]
mod player;

use std::net::UdpSocket;
use std::collections::HashMap;
use ::server::player::Player;
use ::code;

pub const PORT: u16 = 3377;
pub const RECV_SIZE: usize = 256;
pub const DEFAULT_MAX_PLAYERS: usize = 8;
const DEFAULT_LOCAL_PRINT: bool = true;

pub struct Server {
    max_players: usize,
    cur_players: usize,
    name: String,
    width: usize,
    height: usize,
    players: HashMap<String, Player>,
    begin_delimiter: String,
    end_delimiter: String,
    cl_output: bool,
}

// Server Codes:
// 200 login
// 400 logout
// 800 chat
//

impl Server {
    pub fn new() -> Self {
        Server {
            max_players: DEFAULT_MAX_PLAYERS,
            cur_players: 0,
            name: "Default Server Name".to_owned(),
            width: 4,
            height: 4,
            players: HashMap::new(),
            // just visual stuff
            begin_delimiter: "[SERVER] ".to_owned(),
            end_delimiter: "".to_owned(),
            cl_output: DEFAULT_LOCAL_PRINT,
        }
    }
    pub fn start(&mut self) {
        println!("starting server");
        let socket = UdpSocket::bind(("0.0.0.0", PORT)).expect("Cound not create server socket!");

        loop {
            // read from the socket
            let mut buf = [0u8; RECV_SIZE];
            let (_, src) = socket.recv_from(&mut buf)
                .expect("Could not speak with outside world!");
            // send a reply to the socket we received data from
            let (code, content) = match code::split_u8(&buf) {
                Some((c, d)) => (c, d),
                None => ("100", "Split Error"),
            };
            if self.cl_output {
                println!("{}[{}]: {}{}",
                         self.begin_delimiter,
                         code,
                         content,
                         self.end_delimiter);
            }
            let msrc = format!("{}", src).to_owned();
            match &*code {
                "200" => {
                    self.players.entry(msrc).or_insert(Player::new());
                    let isok = socket.send_to("200 login ok".as_bytes(), &src).is_ok();
                    if !isok {
                        println!("{}[{}]: {}{}",
                                 self.begin_delimiter,
                                 code,
                                 "login failed",
                                 self.end_delimiter);
                    }
                }
                "201" => {
                    if self.players.contains_key(&msrc) {
                        let p = self.players.entry(msrc).or_insert(Player::new());
                        p.set_name(&*content);
                    }
                }
                "800" => {
                    match socket.send_to(content.as_bytes(), &src) {
                        Ok(_) => (),
                        Err(e) => println!("{:?}", e),
                    }
                }
                _ => {}
            }
        }
    }
    pub fn output_delim(&mut self, beg: &str, end: &str) {
        self.begin_delimiter = beg.to_owned();
        self.end_delimiter = end.to_owned();
    }
    pub fn disable_output(&mut self) {
        self.cl_output = false;
    }
    pub fn enable_output(&mut self) {
        self.cl_output = true;
    }
}
