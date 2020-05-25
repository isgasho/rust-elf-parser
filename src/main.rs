use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    std::process::exit(run())
}

fn run() -> i32 {
    let result = get_target();
    match result {
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        },
        Ok(target) => {
            println!("{}", target);
            return 0;
        },
    }
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