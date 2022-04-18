use crate::udp::{send_dns_q};
use crate::pkt::{Question};
use std::fs::OpenOptions;
use std::io::Write;

pub mod udp;
mod pkt;

fn main() {
    let question = Question::build(1337,
                                       "www.northeastern.edu",
                                       "A");


    write_to_file("request", question.to_vec());
    let msg = send_dns_q(&question);
    write_to_file("out", msg.to_vec())
}

fn write_to_file(filename: &str, v: Vec<u8>) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename)
        .expect("unable to write to file");

    file.write_all(&v).expect("unable to write to file");

}