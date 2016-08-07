use std::net::UdpSocket;
use std::io;
use std::io::Write;

const PORT: u16 = 3388;

pub struct Client<'a> {
    saddr: &'a str,
    upstream: Option<UdpSocket>,
}

impl<'a> Client<'a> {
    pub fn new() -> Self {
        Client{saddr: r"127.0.0.1", upstream: None}
    }
    pub fn connect(&mut self) {
        let upstream = UdpSocket::bind(("127.0.0.1", PORT))
            .expect("Failed to connect to Socket!");
        self.upstream = Some(upstream);

        let buf = r"200 user login";
        self.send_msg(buf).expect("Couldn't send to");

        if let Ok((buf, _)) = self.recv_msg() {
            let (start, end) = Client::trim(&buf);
            println!("recv: {:?}", String::from_utf8_lossy(&buf[start..end]));
        }
    }
    fn send_msg<'b>(&self, msg: &'b str) -> Result<usize, &'b str> {
        if let Some(ref upstream) = self.upstream {
            match upstream.send_to(msg.as_bytes(), (self.saddr, ::server::PORT)) {
                Err(_) => Err("IO Error while sending data"),
                Ok(c) => Ok(c)
            }
        } else {
            Err("Socket not initialized")
        }
    }
    fn recv_msg(&self) -> Result<([u8; ::server::RECV_SIZE], usize), usize> {
        let mut buf = [0u8; ::server::RECV_SIZE];
        if let Some(ref upstream) = self.upstream {
            match upstream.recv_from(&mut buf) {
                Ok(c) => {
                    Ok((buf, c.0))
                },
                _ => Err(1)
            }
        } else {
            Err(1)
        }

    }
    pub fn control_loop(&mut self) {
        let sin = io::stdin();
        loop {
            print!(" > ");
            io::stdout().flush().expect("Could not flush output");
            let mut msg: String = String::from("800 ");
            match sin.read_line(&mut msg) {
                Ok(c) => println!("Debug: read {:?} bytes", c),
                _ => println!("Debug: error reading input")
            }
            match self.send_msg(&msg[..msg.len()-1]) {
                Ok(c) => println!("Debug: send {:?} bytes", c),
                Err(s) => {println!("Debug: {:?}", s); continue}
            }
            match self.recv_msg() {
                Ok((buf, _)) => {
                    let (start, end) = Client::trim(&buf);
                    println!("{}", String::from_utf8_lossy(&buf[start..end]))
                },
                Err(c) => {println!("Debug: failed to receive answer ({})", c); continue}
            }
        }
    }
    #[allow(dead_code)]
    fn trim(data: &[u8]) -> (usize, usize) {
        let mut start = 0;
        let mut end = data.len();
        let mut switch = false;
        for (k, &v) in data.iter().enumerate() {
            // println!("{} {} {} {}", k, v, start, end);
            if v == 0 {
                if !switch {
                    start = k;
                }
            } else {
                switch = true;
                end = k;
            }
        }
        (start, end+1)
    }
}
