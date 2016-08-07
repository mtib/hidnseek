use std::net::UdpSocket;

const ADDR: &'static str = r"127.0.0.1";
pub const PORT: u16 = 3377;

pub struct Server {
    
}

impl Server {
    pub fn start(&mut self) {
        loop {
            let socket = UdpSocket::bind((ADDR, PORT))
                .expect("Cound not create server socket!");

            // read from the socket
            let mut buf = [0; 10];
            let (amt, src) = socket.recv_from(&mut buf)
                .expect("Could not speak with outside world!");

            // send a reply to the socket we received data from
            let buf = &mut buf[..amt];
            buf.reverse();
            let _ = socket.send_to(buf, &src);
        }
    }
}

pub fn new() -> Server{
    Server{}
}
