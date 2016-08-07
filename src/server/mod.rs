use std::net::UdpSocket;

const ADDR: &'static str = r"127.0.0.1";
pub const PORT: u16 = 3377;

pub struct Server {

}

impl Server {
    pub fn new() -> Self{
        Server{}
    }
    pub fn start(&mut self) {
        let socket = UdpSocket::bind((ADDR, PORT))
            .expect("Cound not create server socket!");

        loop {
            // read from the socket
            let mut buf = [0; 10];
            let (amt, src) = socket.recv_from(&mut buf)
                .expect("Could not speak with outside world!");

            // send a reply to the socket we received data from
            let buf = &mut buf[..amt];
            buf.reverse();
            match socket.send_to(buf, &src){
                Ok(_) => (),
                Err(e) => println!("{:?}", e)
            }
        }
    }
}
