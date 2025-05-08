use std::{ops::Range, slice::Iter};

use bitvec::{order::Lsb0, slice::BitSlice, view::BitView};

use crate::mem;

pub struct RowIter {
    curr: usize,
    n: usize,
    row_start_i: usize,
    this_row_len: usize,
    full_row_len: usize,
}

impl RowIter {
    pub fn new(
        this_row_len: usize,
        full_row_len: usize,
        start_x: usize,
        start_y: usize,
        n: usize,
    ) -> Self {
        let row_start_i = start_y * full_row_len + start_x;
        Self {
            curr: 0,
            full_row_len,
            n,
            row_start_i,
            this_row_len,
        }
    }
}

impl Iterator for RowIter {
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.n {
            let res = self.row_start_i..self.row_start_i + self.this_row_len;
            self.curr += 1;
            self.row_start_i += self.full_row_len;
            return Some(res);
        }
        None
    }
}

pub struct LimitedBitIter<'a> {
    byte_iter: Iter<'a, u8>,
    curr: usize,
    n: usize,
}

impl<'a> LimitedBitIter<'a> {
    pub fn new(mem: &'a mem::Memory, rows: usize) -> Self {
        Self {
            byte_iter: mem.index_iter(),
            curr: 0,
            n: rows,
        }
    }
}

impl<'a> Iterator for LimitedBitIter<'a> {
    type Item = &'a BitSlice<Lsb0, u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.n {
            return match self.byte_iter.next() {
                Some(b) => {
                    self.curr += 1;
                    Some(b.view_bits())
                }
                None => None,
            };
        }
        None
    }
}
