use sha1::{Sha1, Digest};
use flate2::Compression;
use flate2::bufread::ZlibEncoder;
use std::vec::Vec;
use std::fs;
use std::env;
use std::io::prelude::*;
use std::io::{BufWriter, Write, BufReader};

fn calc_hash(_type: &String, body: &String) -> String {
    let len = body.len();
    let mut hasher = Sha1::new();
    hasher.input(format!("{} {}\0{}", _type, len, body));
    hasher.result()
        .iter()
        .map(|i|format!("{:x}",i))
        .collect::<Vec<_>>()
        .join("")
}

// Opens sample file, compresses the contents and returns a Vector or error
// File implements Read
fn open_hello_world() -> std::io::Result<Vec<u8>> {
    let f = fs::File::open("commit.txt")?;
    let b = BufReader::new(f);
    let mut z = ZlibEncoder::new(b, Compression::fast());
    let mut buffer = Vec::new();
    z.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let txt = fs::read_to_string(&args[1]).unwrap();
    println!("{}", calc_hash(&"commit".to_string(), &txt));
    let d = open_hello_world().unwrap();
    let mut f = BufWriter::new(fs::File::create("hello.z").unwrap());
    println!("{:?}", &d);
    f.write(&d).unwrap();
}
