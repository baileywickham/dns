use std::fs::OpenOptions;
use std::io::Write;
use std::net::UdpSocket;
use crate::pkt::{Question, Answer, Message};

pub fn init_conn(addr: &str) -> UdpSocket {
    println!("init conn from: {:?}", addr);
    UdpSocket::bind(addr).expect("Unable to bind socket")
}


pub fn send_dns_q(q : &Question) -> Message {
    let conn = init_conn("0.0.0.0:8080");
    println!("Sending dns request");
    conn.send_to( &q.to_vec(), &"8.8.8.8:53").expect("unable to send pkt");

    let mut buf = [0; 2048];
    let (amt, _src) = conn.recv_from(&mut buf).expect("idk");
    println!("recv bytes: {:?}", amt);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("raw_pkt")
        .expect("unable to write to file");

    file.write_all(&buf[..amt]).expect("unable to write to file");
    Message::read(buf[..amt].to_vec())
}




