extern crate xmas_elf;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;

use xmas_elf::{ElfFile, header};

fn read_file<P: AsRef<Path>>(name: P) -> Vec<u8> {
    let mut f = File::open(name).unwrap();
    let mut buf = Vec::new();
    assert!(f.read_to_end(&mut buf).unwrap() > 0);
    buf
}

fn parse_file<P: AsRef<Path>>(name: P) {
    let buf = read_file(name);
    let elf_file = ElfFile::new(&buf);

    match elf_file.header.pt2.unwrap().type_().as_type() {
        header::Type::Relocatable => println!("this is an object file"),
        _ => println!("not an object file"),
    }
}

fn main() {
    let mut args = env::args();
    let program_name = args.next();

    if let Some(binary_path) = args.next() {
        parse_file(binary_path);
    } else {
        println!("usage: {} <binary_path>", program_name.unwrap());
        exit(1);
    }
}
