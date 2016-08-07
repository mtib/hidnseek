use std::net::UdpSocket;

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
        let result = {
            let upstream = UdpSocket::bind(("127.0.0.1", PORT))
                .expect("Failed to connect to Socket!");

            let buf = b"Hallo Welt you bastard";
            upstream.send_to(buf, (self.saddr, ::server::PORT)).expect("Couldn't send to");

            let mut buf = [0u8; ::server::RECV_SIZE];
            upstream.recv_from(&mut buf).expect("Couldn't receive from");
            let r = buf.into_iter().filter(|&x| *x != 0).map(|x| *x).collect::<Vec<u8>>();
            println!("recv: {:?}", String::from_utf8_lossy(&r));

            self.upstream = Some(upstream);
            r
        };
        println!("returned {:?}", result);
    }
}
