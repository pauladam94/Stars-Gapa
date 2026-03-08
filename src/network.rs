use std::io::Read;

pub fn do_connection() {
    let addr = "127.0.0.8";
    match std::net::TcpListener::bind(addr) {
        Ok(tcp_listener) => (),
        Err(_) => (),
    }

    let mut tcp_stream = match std::net::TcpStream::connect(addr) {
        Ok(tcp_stream) => tcp_stream,
        Err(_) => panic!(),
    };

    let mut buf: [u8; _] = [0; 1000];

    match tcp_stream.read(&mut buf) {
        Ok(i) => todo!(),
        Err(_) => todo!(),
    }
}
