use std::net::UdpSocket;
use std::io::{self, Write};
use std::time::Duration;
use ::code;

const PORT: u16 = 3388;
const DEFAUTL_DEBUG_PRINT: bool = true;

pub struct Client {
    saddr: String,
    upstream: Option<UdpSocket>,
    // debug fields
    debug_output: bool,
}

impl Client {
    pub fn new(saddr: String) -> Self {
        Client {
            saddr: saddr,
            upstream: None,
            debug_output: DEFAUTL_DEBUG_PRINT,
        }
    }
    pub fn connect(&mut self) -> bool {
        println!("connecting to server");
        let upstream = {
            fn get_socket(c: u16) -> UdpSocket {
                let u = UdpSocket::bind(("0.0.0.0", PORT+c)).ok();
                if let Some(s) = u {
                    s
                } else {
                    get_socket(c+1)
                }
            }
            get_socket(0)
        };
        upstream.set_read_timeout(Some(Duration::new(2, 0)))
            .expect("couldn't set socket reat timeout");
        self.upstream = Some(upstream);

        let buf = r"200 user login";
        self.send_msg(buf)
            .expect("failed sending data");

        if let Ok((buf, _)) = self.recv_msg() {
            let (start, end) = code::trim(&buf);
            let (code, content) = code::split_u8(&buf[start..end]).unwrap();
            println!("answer: {}", content);
            if code == "200" {
                self.set_username();
            }
            true
        } else {
            println!("the server didn't respond in time");
            false
        }
    }
    fn send_msg(&self, msg: &str) -> Result<usize, String> {
        if let Some(ref upstream) = self.upstream {
            match upstream.send_to(msg.as_bytes(), (&*self.saddr, ::server::PORT)) {
                Err(e) => Err(format!("{:?}", e)),
                Ok(c) => Ok(c),
            }
        } else {
            Err("Socket not initialized".to_owned())
        }
    }
    fn recv_msg(&self) -> Result<([u8; ::server::RECV_SIZE], usize), usize> {
        let mut buf = [0u8; ::server::RECV_SIZE];
        if let Some(ref upstream) = self.upstream {
            match upstream.recv_from(&mut buf) {
                Ok(c) => Ok((buf, c.0)),
                _ => Err(1),
            }
        } else {
            Err(1)
        }

    }
    pub fn control_loop(&mut self) {
        let sin = io::stdin();
        'input: loop {
            print!(" > ");
            io::stdout().flush().expect("Could not flush output");
            let mut msg: String = "800 ".to_owned();
            match sin.read_line(&mut msg) {
                Ok(c) => {
                    if c == 0 {
                        break 'input;
                    }
                    self.debug_print(format!("Debug: read {:?} bytes", c));
                },
                _ => self.debug_print(format!("Debug: error reading input")),
            }
            if let "quit\n" = &msg[4..] {
                break;
            }
            let whitespace_only = msg.clone().chars().skip(3).all(|x| match x {
                '\n' | '\t' | '\r' | ' ' => true,
                _ => false,
            });
            if whitespace_only {
                self.debug_print("Debug: not sending".to_owned());
                continue 'input;
            }
            match self.send_msg(&msg[..msg.len() - 1]) {
                Ok(c) => self.debug_print(format!("Debug: send {:?} bytes", c)),
                Err(s) => {
                    self.debug_print(format!("Debug: {:?}", s));
                    continue;
                }
            }
            match self.recv_msg() {
                Ok((buf, _)) => {
                    let (start, end) = code::trim(&buf);
                    println!(">>> {}", String::from_utf8_lossy(&buf[start..end]))
                }
                Err(c) => {
                    self.debug_print(format!("Debug: failed to receive answer ({})", c));
                    continue;
                }
            }
        }
        self.send_msg("400 logout").unwrap();
    }
    fn debug_print(&self, s: String) {
        if self.debug_output {
            println!("{}", s);
        }
    }
    fn set_username(&self) {
        print!("username: ");
        let mut username = "201 ".to_owned();
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut username) {
            Ok(_) => {
                let username = username.chars().take(username.len() - 1).collect::<String>();
                self.send_msg(&*username).is_ok();
            }
            Err(_) => println!("setting your username failed"),
        }
    }
    pub fn enable_debug_output(&mut self) {
        self.debug_output = true;
    }
    pub fn disable_debug_output(&mut self) {
        self.debug_output = false;
    }
}
