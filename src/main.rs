use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    run();
}

fn run() -> Result<(), String> {
    let target = get_target()?;

    let mut buf = Vec::new();
    read_elf(&target, &mut buf)?;
    println!("{:?}", buf.get(0..4));
    Ok(())
}

fn get_target() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return Err("Parse error: rust-elf-parser <ELF>".to_string());
    }

    let target = &args[1];
    return Ok(target.to_string());
}

fn read_elf(path: &str, buf: &mut Vec<u8>) -> Result<(), Box<std::error::Error>> {
    let mut file = File::open(Path::new(path))?;
    let _ = file.read_to_end(buf)?;
    Ok(())
}