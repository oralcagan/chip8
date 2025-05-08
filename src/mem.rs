use std::{ops::Range, slice::Iter};

const REG_SIZE: usize = 0x10;
const MEM_SIZE: usize = 0x1000;
const INSTRUCTION_SIZE: usize = 2;

pub struct Memory {
    index: usize,
    pc: usize,
    stack: Vec<u16>,
    mem: Vec<u8>,
    reg: Vec<u8>,
}

impl Memory {
    pub fn new(program_start: usize) -> Self {
        let mem = vec![0; MEM_SIZE];
        let reg = vec![0; REG_SIZE];
        let stack = Vec::new();
        Self {
            index: 0,
            mem,
            reg,
            stack,
            pc: program_start,
        }
    }

    pub fn inst(&self) -> Option<u16> {
        if self.pc > MEM_SIZE - INSTRUCTION_SIZE {
            return None;
        }
        let inst = u8_slice_to_u16_be(&self.mem[self.pc..self.pc + INSTRUCTION_SIZE]);
        Some(inst)
    }

    pub fn move_to_next_inst(&mut self) {
        self.pc += INSTRUCTION_SIZE;
    }

    pub fn move_to_last_inst(&mut self) {
        self.pc -= INSTRUCTION_SIZE;
    }

    pub fn set_pc(&mut self, new_pc: usize) {
        self.pc = new_pc;
    }

    pub fn push_to_stack(&mut self, n: u16) {
        self.stack.push(n)
    }

    pub fn pop_from_stack(&mut self) -> Option<u16> {
        self.stack.pop()
    }

    pub fn slice_from_mem(&self, r: Range<usize>) -> &[u8] {
        &self.mem[r]
    }

    pub fn set_mem_from_slice(&mut self, r: Range<usize>, new_slice: &[u8]) {
        let s = &mut self.mem[r];
        s.copy_from_slice(new_slice);
    }

    pub fn set_index(&mut self, new_index: usize) {
        self.index = new_index;
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn index_iter(&self) -> Iter<u8> {
        self.mem[self.index..].iter()
    }

    pub fn set_reg(&mut self, i: usize, new_val: u8) {
        self.reg[i] = new_val;
    }

    pub fn reg(&self, i: usize) -> u8 {
        self.reg[i]
    }

    pub fn mem_mut(&mut self) -> &mut [u8] {
        &mut self.mem
    }

    pub fn mem(&self) -> &[u8] {
        &self.mem
    }
}

pub fn u8_slice_to_u16_be(s: &[u8]) -> u16 {
    let mut arr: [u8; 2] = Default::default();
    arr.copy_from_slice(s);
    u16::from_be_bytes(arr)
}
