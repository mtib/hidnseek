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

#[derive(Debug)]
pub struct Server {
    max_players: usize,
    cur_players: usize,
    name: String,
    config: Configuration,
    begin_delimiter: String,
    end_delimiter: String,
    cl_output: bool,
}

#[derive(Debug)]
pub struct Configuration {
    width: u32,
    height: u32,
    players: HashMap<String, Player>,
}

// Server Codes:
// 100 split error
// 200 login
// 201 username
// 400 logout
// 800 chat

impl Server {
    pub fn new() -> Self {
        Server {
            max_players: DEFAULT_MAX_PLAYERS,
            cur_players: 0,
            name: "Default Server Name".to_owned(),
            config: Configuration {
                width: 4,
                height: 4,
                players: HashMap::new(),
            },
            // just visual stuff
            begin_delimiter: "[SERVER] ".to_owned(),
            end_delimiter: "".to_owned(),
            cl_output: DEFAULT_LOCAL_PRINT,
        }
    }
    pub fn start(&mut self) {
        println!("starting server");
        let socket = UdpSocket::bind(("0.0.0.0", PORT))
            .expect("Could not bind to socket");

        loop {
            // read from the socket
            let mut buf = [0u8; RECV_SIZE];
            let (_, src) = socket.recv_from(&mut buf)
                .expect("Could not speak with outside world!");
            // send a reply to the socket we received data from
            let (begin, end) = code::trim(&buf);
            let (code, content) = match code::split_u8(&buf[begin..end]) {
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
                    let mut r = Player::new();
                    unsafe {
                        let x = &*self;
                        &r.give_upstream(x as *const Server);
                    }
                    self.config.players.entry(msrc).or_insert(r);
                    let isok = socket.send_to("200 login ok".as_bytes(), &src).is_ok();
                    if !isok {
                        println!("{}[{}]: {}{}",
                                 self.begin_delimiter,
                                 code,
                                 "login failed",
                                 self.end_delimiter);
                    } else {
                        self.cur_players += 1;
                    }
                }
                "201" => {
                    if self.config.players.contains_key(&msrc) {
                        let p = self.config.players.entry(msrc).or_insert(Player::new());
                        p.set_name(&*content);
                    }
                }
                "400" => {
                    self.config.players.remove(&msrc);
                    self.cur_players -= 1;
                }
                "800" => {
                    match socket.send_to(content.as_bytes(), &src) {
                        Ok(_) => (),
                        Err(e) => println!("{:?}", e),
                    }
                }
                _ => {}
                // end of match
            }
            // Debug only:
            for (k, v) in &self.config.players {
                println!("|- {} -> {}", k, v);
            }
            // end of loop
        }
        // end of start
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
