extern crate core;

use crate::udp::{send_dns_q};
use std::fs::OpenOptions;
use std::io::Write;
use bitvec::bitvec;
use bitvec::order::Msb0;
use crate::pkt::message::Message;
use crate::pkt::Serializable;

pub mod udp;
pub mod pkt;

fn main() {
    let message = Message::build(1337,
                                       "www.northeastern.edu",
                                       "A");

    let mut bv = bitvec![u8, Msb0;];
    message.serialize(&mut bv);
    let vector = bv.into_vec();
    write_to_file("request", &vector);
    let m = send_dns_q(&vector);
    println!("{}", m);
    //write_to_file("out", rsp.to_vec())
}

fn write_to_file(filename: &str, v: &Vec<u8>) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename)
        .expect("unable to write to file");

    file.write_all(&v).expect("unable to write to file");

}