use um32::memory::*;
use um32::ops::Op;

fn main () -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let array = array_from_file(&args[1])?;
    for (i, instr) in array.iter().enumerate() {
        println!("{i:#08x}: {:x?}", Op::from(*instr));
    }
    Ok(())
}
