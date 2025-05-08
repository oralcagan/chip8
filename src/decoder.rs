use core::panic;

#[non_exhaustive]
pub enum Instruction {
    ClearScreen,
    Jump { nnn: usize },
    SetReg { x: usize, nn: usize },
    AddToReg { x: usize, nn: usize },
    SetIndex { nnn: usize },
    Display { x: usize, y: usize, n: usize },
    LeaveSubr,
    EnterSubr { nnn: usize },
    SkipIfEq { x: usize, nn: usize },
    SkipIfNEq { x: usize, nn: usize },
    SkipIfEqXY { x: usize, y: usize },
    SkipIfNEqXY { x: usize, y: usize },
    AddNNToX {x : usize, nn : usize},
    Unsupported,
}

pub fn decode_chip8_instruction(inst: u16) -> Instruction {
    match nibble(0, inst) {
        0x0 => match byte(1, inst) {
            0xE0 => Instruction::ClearScreen,
            0xEE => Instruction::LeaveSubr,
            _ => Instruction::Unsupported,
        },
        0x1 => {
            let nnn = three_nibbles(inst) as usize;
            Instruction::Jump { nnn }
        }
        0x2 => {
            let nnn = three_nibbles(inst) as usize;
            Instruction::EnterSubr { nnn }
        }
        0x3 => {
            let x = nibble(1, inst) as usize;
            let nn = byte(1, inst) as usize;
            Instruction::SkipIfEq { x, nn }
        }
        0x4 => {
            let x = nibble(1, inst) as usize;
            let nn = byte(1, inst) as usize;
            Instruction::SkipIfNEq { x, nn }
        }
        0x5 => {
            let x = nibble(1, inst) as usize;
            let y = nibble(2, inst) as usize;
            Instruction::SkipIfEqXY { x, y }
        }
        0x6 => {
            let x = nibble(1, inst) as usize;
            let nn = byte(1, inst) as usize;
            Instruction::SetReg { x, nn }
        }
        0x7 => {
            let x = nibble(1, inst) as usize;
            let nn = byte(1, inst) as usize;
            Instruction::AddToReg { x, nn }
        }
        0x9 => {
            let x = nibble(1, inst) as usize;
            let y = nibble(2, inst) as usize;
            Instruction::SkipIfNEqXY { x, y }
        }
        0xA => {
            let nnn = three_nibbles(inst) as usize;
            Instruction::SetIndex { nnn }
        }
        0xD => {
            let x = nibble(1, inst) as usize;
            let y = nibble(2, inst) as usize;
            let n = nibble(3, inst) as usize;
            Instruction::Display { x, y, n }
        }
        _ => Instruction::Unsupported,
    }
}

fn nibble(i: usize, n: u16) -> u16 {
    match i {
        0 => (n & 0xF000) >> 12,
        1 => (n & 0x0F00) >> 8,
        2 => (n & 0x00F0) >> 4,
        3 => (n & 0x000F),
        _ => panic!("i can't be bigger than 3"),
    }
}

fn byte(i: usize, n: u16) -> u16 {
    match i {
        0 => (n & 0xFF00) >> 8,
        1 => (n & 0xFF),
        _ => panic!("i can't be bigger than 1"),
    }
}

fn three_nibbles(n: u16) -> u16 {
    n & 0xFFF
}
