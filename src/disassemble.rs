use std::{fs::File, io::BufReader};

use intel8080::Memory;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let file = File::open(&args[1])?;
    let mut reader = BufReader::new(file);

    let mut mem = Memory::from_reader(&mut reader)?;

    while (mem.pc as usize) < mem.bytes_read {
        let pc = mem.pc;
        let opcode = mem.read_opcode();
        println!("{:#010x}: {:?}", pc, opcode);
    }

    Ok(())
}
