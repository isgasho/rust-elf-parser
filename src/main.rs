use std::env;

fn main() {
    std::process::exit(run())
}

fn run() -> i32 {
    let result = getTarget();
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

fn getTarget() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return Err("Parse error: rust-elf-parser <ELF>".to_string());
    }

    let target = &args[1];
    return Ok(target.to_string());
}
