fn main() {
    use std::net::UdpSocket;
    use std::env;

    fn serve() -> std::io::Result<()> {
        {
            let socket = try!(UdpSocket::bind("127.0.0.1:34254"));

            // read from the socket
            let mut buf = [0; 10];
            let (amt, src) = try!(socket.recv_from(&mut buf));

            // send a reply to the socket we received data from
            let buf = &mut buf[..amt];
            buf.reverse();
            try!(socket.send_to(buf, &src));
        Ok(())
        } // the socket is closed here
    }

    fn client() {
        let csocket = UdpSocket::bind("127.0.0.1:34255").expect("Failed to connect to Socket!");
        let buf = "Hallo Welt".as_bytes();
        let _ = csocket.send_to(&buf, "127.0.0.1:34254");
        let mut buf = vec![0u8; buf.len()];
        println!("{:?}", buf);
        let _ = csocket.recv_from(&mut buf);
        println!("recv: {}", String::from_utf8(buf).expect("didn't receive valid utf8"));
    }

    match env::args().nth(1).unwrap().as_str() {
        "s" => loop { let _ = serve(); },
        "c" => client(),
        o => println!("use s or c, not: {:?}", o),
    }
}
