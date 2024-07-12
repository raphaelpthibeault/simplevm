use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

pub fn main() -> Result<(), String> {
    // ./asm file.asm

    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: {} <input>", args[0]);
    }

    let file =
        File::open(Path::new(&args[1])).map_err(|x| format!("Failed to open file: {}", x))?;

    // foreach line in file
    //  foreach space-separated token
    //      attempt to parse as base-16 number
    //      append to output if number
    //      else die
    // e.g.
    // 00 01 02 03 04 05 10
    let mut output: Vec<u8> = Vec::new();
    for line in BufReader::new(file).lines() {
        let line_inner = line.map_err(|x| format!("read line: {}", x))?;
        for t in line_inner.split(" ").filter(|x| x.len() > 0) {
            let b = u8::from_str_radix(t, 16).map_err(|x| format!("parse int: {}", x))?;
            output.push(b);
        }
    }
    let mut stdout = io::stdout().lock();
    stdout
        .write_all(&output)
        .map_err(|x| format!("write all: {}", x))?;
    Ok(())
}
