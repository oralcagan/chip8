use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal::{self, ClearType},
    ExecutableCommand, QueueableCommand,
};
use std::io::{self, Write};

use crate::util::RowIter;

pub struct MonochromeDisplay {
    stdout: io::Stdout,
    active_pixel: char,
    inactive_pixel: char,
    width: u16,
    height: u16,
    title: String,
    img: Vec<bool>,
}

impl MonochromeDisplay {
    pub fn try_new(
        active_pixel: char,
        inactive_pixel: char,
        width: u16,
        height: u16,
        title: String,
    ) -> Result<Self, io::Error> {
        let mut stdout = io::stdout();
        stdout
            .execute(terminal::SetTitle(&title))?
            .execute(terminal::SetSize(width, height))?
            .execute(cursor::Hide)?
            .execute(terminal::Clear(ClearType::All))?;
        let img = vec![false; (width * height) as usize];
        Ok(Self {
            width,
            height,
            title,
            active_pixel,
            stdout,
            inactive_pixel,
            img,
        })
    }

    pub fn write(&mut self, s: &str) {
        self.stdout
            .execute(cursor::MoveTo(32, 31))
            .unwrap()
            .execute(style::Print(s))
            .unwrap();
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: &str) -> Result<(), io::Error> {
        self.stdout.execute(terminal::SetTitle(title))?;
        Ok(())
    }

    pub fn dimensions(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub fn img(&self) -> &[bool] {
        &self.img
    }

    pub fn img_mut(&mut self) -> &mut [bool] {
        &mut self.img
    }

    pub fn q_modify_pixel(&mut self, row: usize, col: usize, state: bool) -> Result<(), io::Error> {
        self.stdout.queue(cursor::MoveTo(col as u16, row as u16))?;
        if state {
            self.stdout
                .queue(style::Print(self.active_pixel.yellow()))?;
            return Ok(());
        }
        self.stdout
            .queue(style::PrintStyledContent(self.inactive_pixel.dark_grey()))?;
        Ok(())
    }

    pub fn draw(
        &mut self,
        start_x: usize,
        start_y: usize,
        width: usize,
        height: usize,
    ) -> Result<(), io::Error> {
        let rows = RowIter::new(width, self.width as usize, start_x, start_y, height);
        for (i, row_r) in rows.enumerate() {
            for (j, pixel) in row_r.enumerate() {
                self.q_modify_pixel(start_y + i, start_x + j, self.img[pixel])?;
            }
        }
        self.flush()?;
        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), io::Error> {
        self.stdout.execute(terminal::Clear(ClearType::Purge))?;
        self.img.fill(false);
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), io::Error> {
        Ok(self.stdout.flush()?)
    }
}
