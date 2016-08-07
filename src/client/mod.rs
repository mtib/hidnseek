use std::net::UdpSocket;
use server;

const PORT: u16 = 3388;

pub struct Client<'a> {
    saddr: &'a str,
    upstream: Option<UdpSocket>,
}

impl<'a> Client<'a> {
    pub fn connect(&'a mut self) {
        let result = {
            self.upstream = Some(UdpSocket::bind(("127.0.0.1", PORT))
                .expect("Failed to connect to Socket!"));
            let s = self.upstream.as_ref().unwrap();
            let buf = b"Hallo Welt";
            let _ = s.send_to(buf, (self.saddr, server::PORT));
            let mut buf = vec![0u8; buf.len()];
            println!("{:?}", buf);
            let _ = s.recv_from(&mut buf);
            let r = String::from_utf8(buf).expect("didn't receive valid utf8");
            println!("recv: {}", r);
            r
        };
        println!("returned {:?}", result);
    }
}

pub fn new<'a>() -> Client<'a> {
    Client{saddr: r"127.0.0.1", upstream: None}
}
