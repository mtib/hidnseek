use std::net::UdpSocket;
use std::io::{self, Write};
use std::time::Duration;
use ::code;

/// Client Port, if used will be incremented
const PORT: u16 = 3388;
/// Boolean whether or not to print debug messages to cli
const DEFAULT_DEBUG_PRINT: bool = true;
// TODO writing a log / error file

/// Local client, run in the terminal, with which the user interacts.
/// Not to confuse with ::server::Player , which is the Servers modeling
/// of the clients and their position.
/// The clients do not know of each other.
pub struct Client {
    saddr: String,
    upstream: Option<UdpSocket>,
    // debug fields
    debug_output: bool,
}

impl Client {
    /// saddr: the udp address of the server
    pub fn new(saddr: String) -> Self {
        Client {
            saddr: saddr,
            upstream: None,
            debug_output: DEFAULT_DEBUG_PRINT,
        }
    }
    /// Login into a server, after the Client has been
    /// created. It also handles choosing a username.
    pub fn connect(&mut self) -> bool {
        println!("connecting to server");
        // getting a local socket, port can be anything
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
        // timeout after 2 seconds.
        upstream.set_read_timeout(Some(Duration::new(2, 0)))
            .expect("couldn't set socket read timeout");
        self.upstream = Some(upstream);

        // tell the server, that you are connecting
        let buf = r"200 user login";
        self.send_msg(buf)
            .expect("failed sending data");

        if let Ok((buf, _)) = self.recv_msg() {
            let (start, end) = code::trim(&buf);
            let (code, content) = code::split_u8(&buf[start..end]).unwrap();
            println!("answer: {}", content);
            if code == "200" {
                // on succesful connect: set username
                // the server already knows about the client,
                // and keeps track by <ipaddr>:<port>
                // the default username is "Anonymus"
                self.set_username();
            }
            true
        } else {
            println!("the server didn't respond in time");
            // the connection could still be working tho.
            false
        }
    }

    /// Sends the message to self.saddr, on the hardcoded ::server::PORT
    fn send_msg(&self, msg: &str) -> Result<usize, String> {
        if let Some(ref upstream) = self.upstream {
            match upstream.send_to(msg.as_bytes(), (&*self.saddr, ::server::PORT)) {
                Err(e) => Err(format!("{:?}", e)),
                Ok(c) => Ok(c),
            }
        } else {
            // you need to call self.connect() before hand
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
            // error codes don't really mean much rn.
            // TODO better error codes for receiving messages
            Err(1)
        }

    }

    /// This is the input loop for the user,
    /// it also handles all special cases 200-800
    pub fn control_loop(&mut self) {
        let sin = io::stdin();
        'input: loop {
            print!(" > ");
            io::stdout().flush().expect("Could not flush output");
            // 800: send by terminal (not by software routine)
            // everything entered by a user will be 8XX,
            // movement and other key words will get 81X or higher.
            let mut msg: String = "800 ".to_owned();
            match sin.read_line(&mut msg) {
                Ok(c) => {
                    if c == 0 {
                        // empty string was read:
                        // only possible with Escape Sequences
                        // or by piping input into the program.
                        // Piping might be useful for testing
                        // purposes.
                        break 'input;
                    }
                    self.debug_print(format!("Debug: read {:?} bytes", c));
                },
                _ => self.debug_print(format!("Debug: error reading input")),
            }
            // quit also tries to logout, later on
            if let "quit\n" = &msg[4..] {
                break;
            }
            // do not send whitespace_only messages.
            let whitespace_only = msg.clone().chars().skip(3).all(|x| match x {
                '\n' | '\t' | '\r' | ' ' => true,
                _ => false,
            });
            if whitespace_only {
                self.debug_print("Debug: not sending".to_owned());
                continue 'input;
            }
            // sending "800 " + what the user has entered to the server
            // without the final \n, but with all other escaped chars.
            match self.send_msg(&msg[..msg.len() - 1]) {
                Ok(c) => self.debug_print(format!("Debug: send {:?} bytes", c)),
                Err(s) => {
                    self.debug_print(format!("Debug: {:?}", s));
                    continue;
                }
            }
            // the client usually gets an answer from the server
            // this could be where we check if the UDP packet reached
            // the server.
            match self.recv_msg() {
                Ok((buf, _)) => {
                    let (start, end) = code::trim(&buf);
                    println!(">>> {}", String::from_utf8_lossy(&buf[start..end]))
                }
                Err(c) => {
                    // instead of doing this we could send the message
                    // again, awaiting another ACK, after n tries the
                    // client should not longer try to send data.
                    self.debug_print(format!("Debug: failed to receive answer ({})", c));
                    continue;
                }
            }
        }
        // the client sending it's logout, to the server can remove it
        // from it's Configuration.players HashMap.
        // TODO also send on Interrupt or SIGKILL if possible
        self.send_msg("400 logout").unwrap();
    }
    /// Prints to the commandline if self.debug_output is true.
    /// Use {en,dis}able_debug_output() to change.
    fn debug_print(&self, s: String) {
        if self.debug_output {
            println!("{}", s);
        }
    }
    /// This function asks for the username and tries to send it.
    /// The client has to be connected for this to work.
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
    /// Enables output using the function debug_print.
    pub fn enable_debug_output(&mut self) {
        self.debug_output = true;
    }
    /// Disables output using the function debug_print.
    pub fn disable_debug_output(&mut self) {
        self.debug_output = false;
    }
}
