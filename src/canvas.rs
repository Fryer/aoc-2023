use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};
use crossterm::{execute, queue};
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::Event::Key;
use crossterm::event::{poll, read};
use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, size};
use ndarray::{Array, Array2};

pub type Color = crossterm::style::Color;

pub trait IntoColor {
    fn into(self) -> Color;
}

impl IntoColor for Color {
    fn into(self) -> Color {
        return self;
    }
}

impl IntoColor for (u8, u8, u8) {
    fn into(self) -> Color {
        let (r, g, b) = self;
        return Color::Rgb { r, g, b };
    }
}

pub struct Canvas {
    buffer: Array2<(char, Color, Color)>,
    old_buffer: Array2<(char, Color, Color)>,
    last_update: Instant,
    style: (Color, Color),
}

impl Canvas {
    pub fn new() -> Canvas {
        execute!(stdout(), EnterAlternateScreen).unwrap();
        let (w, h) = size().unwrap();
        let shape = (h as usize, w as usize);
        let buffer = Array::from_elem(shape, (' ', Color::Reset, Color::Reset));
        let old_buffer = buffer.clone();
        let last_update = Instant::now();
        let style = (Color::Reset, Color::Reset);
        return Canvas { buffer, old_buffer, last_update, style };
    }

    pub fn size(&self) -> (usize, usize) {
        return (self.buffer.nrows(), self.buffer.ncols());
    }

    pub fn begin(&mut self, step: u64) {
        let duration = Duration::from_millis(step);
        let elapsed = self.last_update.elapsed();
        if elapsed < duration {
            sleep(duration - elapsed);
            self.last_update += duration;
        }
        if elapsed > duration * 2 {
            self.last_update += elapsed;
        }
        let (w, h) = size().unwrap();
        let shape = (h as usize, w as usize);
        if self.buffer.shape() != [shape.0, shape.1] {
            self.buffer = Array::from_elem(shape, (' ', self.style.0, self.style.1));
            self.old_buffer = Array::from_elem(shape, ('\0', self.style.0, self.style.1));
        }
        else {
            self.buffer.fill((' ', self.style.0, self.style.1));
        }
    }

    pub fn end(&mut self) {
        let mut lfg = Color::White;
        let mut lbg = Color::Black;
        let mut skip = true;
        for (r, row) in self.buffer.rows().into_iter().enumerate() {
            for (c, &(ch, fg, bg)) in row.iter().enumerate() {
                let old = self.old_buffer[(r, c)];
                if (ch, fg, bg) == old {
                    skip = true;
                    continue;
                }
                if skip {
                    queue!(stdout(), MoveTo(c as u16, r as u16)).unwrap();
                    skip = false;
                }
                if lfg != fg && fg != old.1 {
                    queue!(stdout(), SetForegroundColor(fg)).unwrap();
                    lfg = fg;
                }
                if lbg != bg && bg != old.2 {
                    queue!(stdout(), SetBackgroundColor(bg)).unwrap();
                    lbg = bg;
                }
                queue!(stdout(), Print(ch)).unwrap();
            }
        }
        queue!(stdout(), ResetColor, Hide).unwrap();
        stdout().flush().unwrap();
        self.old_buffer = self.buffer.clone();
    }

    pub fn style<F: IntoColor, B: IntoColor>(&mut self, fg: F, bg: B) {
        self.style = (fg.into(), bg.into());
    }

    pub fn reset_style(&mut self) {
        self.style = (Color::Reset, Color::Reset);
    }

    pub fn fg<T: IntoColor>(&mut self, color: T) {
        self.style.0 = color.into();
    }

    pub fn bg<T: IntoColor>(&mut self, color: T) {
        self.style.1 = color.into();
    }

    pub fn draw(&mut self, r: usize, c: usize, ch: char) -> bool {
        if r < self.buffer.nrows() && c < self.buffer.ncols() {
            self.buffer[(r, c)] = (ch, self.style.0, self.style.1);
            return true;
        }
        return false;
    }

    pub fn print(&mut self, r: usize, c: usize, text: &str) {
        for (i, ch) in text.chars().enumerate() {
            if !self.draw(r, c + i, ch) {
                return;
            }
        }
    }

    pub fn pause(&self) {
        while poll(Duration::new(0, 0)).unwrap() {
            read().unwrap();
        }
        while !matches!(read().unwrap(), Key(_)) {}
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        execute!(stdout(), LeaveAlternateScreen, Show).unwrap();
    }
}
