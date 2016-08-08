#![allow(dead_code)]
mod player;

use std::net::UdpSocket;
use std::collections::HashMap;
use ::server::player::Player;
use ::code;

/// Server port, hardcoded
pub const PORT: u16 = 3377;
/// Message buffer size
pub const RECV_SIZE: usize = 256;
/// Maximum players per server
pub const DEFAULT_MAX_PLAYERS: usize = 8;
/// Boolean whether or not to print messages
/// from the local game server, if it is running
/// on the same machine and terminal
const DEFAULT_LOCAL_PRINT: bool = true;

/// Game server
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

/// Everything concerning the map and players
#[derive(Debug)]
pub struct Configuration {
    width: u32,
    height: u32,
    players: HashMap<String, Player>,
}

// Server Codes:
// 1XX # server side errors
// 10X ## message handling error
// 100 split error
// 2XX # setup
// 20X ## connecting
// 200 login
// 201 username
// 3XX # server info to clients
// 4XX # client side errors
// 40X ## leaving
// 400 logout
// 8XX # commands from the clients
// 80X ## non game relevant
// 800 chat
// 81X ## movement

macro_rules! server_print {
    ($slf:ident, $fmt:expr) => {
        println!(concat!("{}", $fmt, "{}"),
                 $slf.begin_delimiter,
                 $slf.end_delimiter);
    };
    ($slf:ident, $fmt:expr, $($arg:tt)*) => {
        println!(concat!("{}", $fmt, "{}"),
                 $slf.begin_delimiter,
                 $($arg)*,
                 $slf.end_delimiter);
    };
}

// TODO server console
// TODO changing server name
impl Server {
    /// Creates a new Server
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
    /// Will run the server in a loop, currently only single threaded
    pub fn start(&mut self) {
        println!("starting server");
        let socket = UdpSocket::bind(("0.0.0.0", PORT))
            .expect("Could not bind to socket");

        // TODO multithreading?
        loop {
            // read from the socket
            let mut buf = [0u8; RECV_SIZE];
            let (_, src) = socket.recv_from(&mut buf)
                .expect("Could not speak with outside world!");

            // take the message appart: buf = "<code> <content>"
            let (begin, end) = code::trim(&buf);
            let (code, content) = match code::split_u8(&buf[begin..end]) {
                Some((c, d)) => (c, d),
                None => ("100", "Split Error"),
            };
            // debug printing
            if self.cl_output {
                server_print!(self, "[{}]: {}", code, content);
            }
            // <addr>:<port> used to identify the user
            let msrc = format!("{}", src).to_owned();
            // TODO move logic into functions
            match &*code {
                // new player connected (login)
                "200" => {
                    let mut r = Player::new();
                    unsafe {
                        let x = &*self;
                        r.give_upstream(x as *const Server);
                    }
                    self.config.players.entry(msrc).or_insert(r);
                    let isok = socket.send_to(b"200 login ok", &src).is_ok();
                    if !isok {
                        server_print!(self, "[{}]: {}", code, "login failed");
                    } else {
                        self.cur_players += 1;
                    }
                }
                // player set their username (201 <username>)
                "201" => {
                    if self.config.players.contains_key(&msrc) {
                        let p = self.config.players.entry(msrc).or_insert_with(Player::new);
                        p.set_name(&*content);
                    }
                }
                // player broke out of control_loop, now logging out
                "400" => {
                    self.config.players.remove(&msrc);
                    self.cur_players -= 1;
                }
                // player sending a normal text message
                // TODO append 800s to players msg as chat?
                "800" => {
                    match socket.send_to(content.as_bytes(), &src) {
                        Ok(_) => (),
                        Err(e) => println!("{:?}", e),
                    }
                }
                _ => {}
                // end of matching <code>
            }
            // Debug only:
            for (k, v) in &self.config.players {
                println!("|- {} -> {}", k, v);
            }
            // end of loop
        }
        // end of start
    }

    /// Setting before and after str for command line output
    pub fn output_delim(&mut self, beg: &str, end: &str) {
        self.begin_delimiter = beg.to_owned();
        self.end_delimiter = end.to_owned();
    }
    /// Disables command line output
    pub fn disable_output(&mut self) {
        self.cl_output = false;
    }
    /// Disables command line output
    pub fn enable_output(&mut self) {
        self.cl_output = true;
    }
}
