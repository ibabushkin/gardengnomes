extern crate xmas_elf;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;

use xmas_elf::{ElfFile, header};
use xmas_elf::sections::ShType;

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
        header::Type::Relocatable => dump_info(elf_file),
        _ => println!("not an object file"),
    }
}

fn dump_info(elf: ElfFile) {
    for section in elf.section_iter().skip(1) {
        match (section.get_name(&elf), section.get_type()) {
            (Ok(name), Ok(t)) => {
                println!("section {}, data:", name);
                dump_section_data(t);
            },
            _ => println!("invalid section <:("),
        }
    }
}

fn dump_section_data(t: ShType) {
    let desc = match t {
        ShType::ProgBits => "progbits binary (code, data,...)",
        ShType::NoBits => "nobits binary (probably data)",
        ShType::Rela | ShType::Rel => "relocations",
        ShType::SymTab => "symbol table",
        ShType::StrTab => "string table",
        _ => "something else",
    };
    println!("\t{}", desc);
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
