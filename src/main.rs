use sha1::{Sha1, Digest};
use flate2::Compression;
use flate2::bufread;
use flate2::write;
use std::vec::Vec;
use std::fs;
use std::env;
use std::io::prelude::*;
use std::io::{BufWriter, Write, BufReader};
use std::path::Path;

fn calc_hash(header: &String, content: &String) -> String {
    let mut hasher = Sha1::new();
    hasher.input(format!("{}{}", header, content));
    hasher.result()
        .iter()
        .map(|i|format!("{:x}",i))
        .collect::<Vec<_>>()
        .join("")
}

fn create_blob(content: &String) -> String {
    let len = content.len();
    let header = format!("blob {}\0", len);
    calc_hash(&header, content)
}

fn zlib_st(content: &String) -> std::io::Result<Vec<u8>> {
    let mut e = flate2::write::ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(&content.as_bytes()).unwrap();
    e.finish()
}

fn zlib_f(filename: &String) -> std::io::Result<Vec<u8>> {
    let f = fs::File::open(&filename)?;
    let b = BufReader::new(f);
    let mut z = flate2::bufread::ZlibEncoder::new(b, Compression::fast());
    let mut buffer = Vec::new();
    z.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn create_hash_object(content: &String) {
    let sha1 = create_blob(&content);
    let compressed_content = zlib_st(content).unwrap();
    let t = format!("./.mygit/objects/{}", &sha1[0..2]);
    let path = Path::new(&t);
    if !path.exists() {
        fs::create_dir_all(path);
    }
    let filepath = t + "/" + &sha1[2..38];
    let mut f = BufWriter::new(fs::File::create(filepath).unwrap());
    f.write(&compressed_content).unwrap();
}

fn main() {
    // 引数を入力
    let args: Vec<String> = env::args().collect();

    if &args[1] == "hash-object-w--stdin" {
        let content = &args[2];
        create_hash_object(content);
    }
}
