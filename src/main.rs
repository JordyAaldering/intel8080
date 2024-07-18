use std::{fs::File, io::BufReader};

use intel8080::{emulate, Flags, Memory, Registers};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let file = File::open(&args[1])?;
    let mut reader = BufReader::new(file);
    let mut mem = Memory::from_reader(&mut reader)?;

    let mut reg = Registers::default();
    let mut flags = Flags::default();

    loop {
        emulate(&mut reg, &mut flags, &mut mem);
    }
}
