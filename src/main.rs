mod core;

use core::state::Chip8State;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).ok_or("usage: chip8 <file_path>")?;
    let state = Chip8State::new_from_rom(path)?;

    for i in (0x200..0x220).step_by(2) {
        println!("{:03X}: {:02X}{:02X}", i, state.memory[i], state.memory[i + 1]);
    }

    Ok(())
}
