use sha1::{Sha1, Digest};
use std::vec::Vec;
use std::fs;
use std::env;

fn calc_hash(body: &String) -> String {
    let len = body.len();
    let mut hasher = Sha1::new();
    hasher.input(format!("commit {}\0{}", len, body));
    hasher.result()
        .iter()
        .map(|i|format!("{:x}",i))
        .collect::<Vec<_>>()
        .join("")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let txt = fs::read_to_string(&args[1]).unwrap();
    println!("{}", calc_hash(&txt));
}
