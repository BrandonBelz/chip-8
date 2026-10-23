use std::fs;

pub const EXEC_START_ADDR: usize = 0x200;

pub struct Chip8State {
    pub memory: [u8; 4096],
    display: [bool; 2048],
    stack: Vec<u16>,
    registers: [u8; 16],
    program_counter: u16,
    index: u16,
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8State {
    pub fn new() -> Self {
        Self {
            memory: [0; 4096],
            display: [false; 2048],
            stack: Vec::new(),
            registers: [0; 16],
            program_counter: EXEC_START_ADDR as u16,
            index: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn new_from_rom(file_path: &str) -> Result<Self, String> {
        let mut state = Self::new();

        let rom = fs::read(file_path).map_err(|e| format!("couldn't read {file_path}: {e}"))?;
        if rom.len() > state.memory.len() - EXEC_START_ADDR {
            return Err(format!("ROM is too large ({} bytes)", rom.len()));
        }

        state.memory[EXEC_START_ADDR..EXEC_START_ADDR + rom.len()].copy_from_slice(&rom);
        Ok(state)
    }
}
