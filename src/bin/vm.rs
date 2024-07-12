use std::{
    env,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use simplevm::{Machine, Register};

pub fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: {} <input>", args[0]);
    }

    let file =
        File::open(Path::new(&args[1])).map_err(|x| format!("Failed to open file: {}", x))?;

    let mut reader = BufReader::new(file);
    let mut program: Vec<u8> = Vec::new();
    reader
        .read_to_end(&mut program)
        .map_err(|x| format!("Read to end: {}", x))?;

    let mut vm = Machine::new();
    vm.memory.load_from_vec(&program, 0);

    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?;
    println!("A = {}", vm.get_register(Register::A));
    Ok(())
}
