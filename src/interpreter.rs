use crate::util::{LimitedBitIter, RowIter};
use crate::{decoder, display, mem};
use std::cmp::min;
use std::fs;
use std::io;

const PROGRAM_START: usize = 0x200;
const DISPLAY_WIDTH: u16 = 64;
const DISPLAY_HEIGHT: u16 = 32;
const WINDOW_TITLE: &str = "CHIP8";
const ACTIVE_PIXEL: char = '█';
const INACTIVE_PIXEL: char = ' ';
const BYTE_LEN: usize = 8;

pub struct Interpreter {
    display: display::MonochromeDisplay,
    mem: mem::Memory,
}

impl Interpreter {
    pub fn try_new(file_path: &str) -> Result<Self, io::Error> {
        let mut mem = mem::Memory::new(PROGRAM_START);
        let inst = load_instructions(file_path)?;
        mem.set_mem_from_slice(PROGRAM_START..PROGRAM_START + inst.len(), &inst);
        let display = display::MonochromeDisplay::try_new(
            ACTIVE_PIXEL,
            INACTIVE_PIXEL,
            DISPLAY_WIDTH,
            DISPLAY_HEIGHT,
            WINDOW_TITLE.to_string(),
        )?;
        Ok(Self { display, mem })
    }

    pub fn run(&mut self) {
        loop {
            let inst = match self.mem.inst() {
                Some(i) => i,
                None => {
                    println!("Program end");
                    return;
                }
            };
            self.mem.move_to_next_inst();
            match decoder::decode_chip8_instruction(inst) {
                decoder::Instruction::ClearScreen => {
                    self.display.clear().unwrap();
                }
                decoder::Instruction::Jump { nnn } => {
                    self.mem.set_pc(nnn);
                }
                decoder::Instruction::SetReg { x, nn } => {
                    self.mem.set_reg(x, nn as u8);
                }
                decoder::Instruction::AddToReg { x, nn } => {
                    let temp = self.mem.reg(x);
                    self.mem.set_reg(x, temp + nn as u8);
                }
                decoder::Instruction::SetIndex { nnn } => {
                    self.mem.set_index(nnn);
                }
                decoder::Instruction::Display { x, y, n } => {
                    self.display(x, y, n);
                }
                decoder::Instruction::Unsupported => {
                    println!("Unsupported instruction {}", inst);
                    return;
                }
                decoder::Instruction::LeaveSubr => todo!(),
                decoder::Instruction::EnterSubr { nnn } => todo!(),
                decoder::Instruction::SkipIfEq { x, nn } => todo!(),
                decoder::Instruction::SkipIfNEq { x, nn } => todo!(),
                decoder::Instruction::SkipIfEqXY { x, y } => todo!(),
                decoder::Instruction::SkipIfNEqXY { x, y } => todo!(),
                decoder::Instruction::AddNNToX { x, nn } => todo!(),
            };
        }
    }

    fn display(&mut self, vx: usize, vy: usize, n: usize) {
        let x = self.mem.reg(vx) as usize % DISPLAY_WIDTH as usize;
        let y = self.mem.reg(vy) as usize % DISPLAY_HEIGHT as usize;
        self.mem.set_reg(0xf, 0);
        let mut sprite_row_iter = LimitedBitIter::new(&self.mem, n);
        let display_row_iter = RowIter::new(
            min(DISPLAY_WIDTH as usize - x, BYTE_LEN),
            self.display.dimensions().0 as usize,
            x,
            y,
            min(DISPLAY_HEIGHT as usize - y, n),
        );
        let mut flip_reg = false;
        for row_r in display_row_iter {
            let sprite_row = sprite_row_iter.next().unwrap();
            for (j, pixel) in row_r.enumerate() {
                let img = self.display.img_mut();
                let temp = img[pixel];
                img[pixel] ^= sprite_row[BYTE_LEN - 1 - j];
                flip_reg = temp && sprite_row[j];
            }
        }
        if flip_reg {
            self.mem.set_reg(0xf, 1)
        }
        self.display.draw(x, y, BYTE_LEN, n).unwrap();
    }
}

fn load_instructions(file_path: &str) -> Result<Vec<u8>, io::Error> {
    fs::read(file_path)
}
