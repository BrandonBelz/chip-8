pub struct Chip8State {
    memory: [u8; 4096],
    display: [bool; 2048],
    stack: [u16; 64],
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
            stack: [0; 64],
            registers: [0; 16],
            program_counter: 0,
            index: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }
}