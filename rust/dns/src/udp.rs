use std::fs::OpenOptions;
use std::io::Write;
use std::net::UdpSocket;
use crate::Message;


pub fn init_conn(addr: &str) -> UdpSocket {
    println!("init conn from: {:?}", addr);
    UdpSocket::bind(addr).expect("Unable to bind socket")
}


pub fn send_dns_q(data : &Vec<u8>) -> Message {
    let conn = init_conn("0.0.0.0:8080");
    println!("Sending dns request");
    conn.send_to( data, &"1.1.1.1:53").expect("unable to send pkt");
    println!("sent request");

    let mut buf = [0; 10000];
    let (amt, _src) = conn.recv_from(&mut buf).expect("problem reading bits");
    println!("recv bytes: {:?}", amt);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("raw_pkt")
        .expect("unable to write to file");

    file.write_all(&buf[..amt]).expect("unable to write to file");
    Message::deserialize(&buf[..amt])
}




