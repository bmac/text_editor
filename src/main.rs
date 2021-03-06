use std::io::prelude::*;
use std::fs::File;
extern crate termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::event::Key;

fn main() {
    
    let mut f = File::open("foo.txt").expect("foo.txt is a fixture and should be defined");
    let mut buffer = String::new();
    let _result = f.read_to_string(&mut buffer);


    let lines: Vec<String> = buffer.split('\n').map(|s| s.to_string()).collect();

    let mut editor = Editor::new(lines);
    
    editor.run();
}


#[derive(Debug)]
struct Editor {
    buffer: Buffer,
    cursor: Cursor,
}

impl Editor {
    fn run(&mut self) {
        let mut stdout = std::io::stdout().into_raw_mode().unwrap();
        
        loop {
            self.render();
            let _result = stdout.flush();
            if self.handle_input() {
                break
            }
        }
    }

    fn render(&self) {
        ANSI::clear_screen();
        ANSI::move_cursor(0, 0);
        self.buffer.render();
        ANSI::move_cursor(self.cursor.row, self.cursor.col);
    }

    fn handle_input(&mut self) -> bool {
        let stdin = std::io::stdin();
        let key = stdin.keys().next().unwrap();

        match key {
            Ok(Key::Ctrl('n')) => self.cursor = self.cursor.down(&self.buffer),
            Ok(Key::Ctrl('p')) => self.cursor = self.cursor.up(&self.buffer),
            Ok(Key::Ctrl('f')) => self.cursor = self.cursor.right(&self.buffer),
            Ok(Key::Ctrl('b')) => self.cursor = self.cursor.left(&self.buffer),
            Ok(Key::Backspace) => {
                if self.cursor.col > 0 {
                    self.buffer = self.buffer.delete(self.cursor.row, self.cursor.col -1 );
                    self.cursor = self.cursor.left(&self.buffer);
                }
            }
            Ok(Key::Char('\n')) => {
                self.buffer = self.buffer.split_line(self.cursor.row, self.cursor.col);
                self.cursor = self.cursor.down(&self.buffer).move_to_col(0);
            }
            Ok(Key::Char(c)) => {
                self.buffer = self.buffer.insert(c, self.cursor.row, self.cursor.col);
                self.cursor = self.cursor.right(&self.buffer);
            }
            _ => ()
        };

        // return true to break the look and exit the program
        return match key {
            Ok(Key::Ctrl('c')) => true,
            Ok(Key::Ctrl('q')) => true,
            _ => false,
        }
    }

    fn new(lines: Vec<String>) -> Editor {
        let buffer = Buffer { lines };
        let cursor = Cursor {
            row: 0,
            col: 0,
        };

        Editor { buffer, cursor }
    }
}

#[derive(Debug)]
struct Buffer {
    lines: Vec<String>
}

impl Buffer {
    fn render(&self) {
        for line in &self.lines {
            print!("{}\r\n", line);
        }
    }

    fn line_count(&self) -> i64 {
        return self.lines.len() as i64;
    }

    fn line_length(&self, row: i64) -> i64 {
        return self.lines[row as usize].len() as i64;
    }

    fn insert(&self, character: char, row: i64, col: i64) -> Buffer {
        let mut lines = self.lines.to_vec();
        lines[row as usize].insert(col as usize, character);
        return Buffer { lines };
    }

    fn delete(&self, row: i64, col: i64) -> Buffer {
        let mut lines = self.lines.to_vec();
        let start = col as usize;
        lines[row as usize].drain(start..start+1);
        return Buffer { lines };
    }

    fn split_line(&self, row: i64, col: i64) -> Buffer {
        let mut lines = self.lines.to_vec();
        let line = &self.lines[row as usize];
        let (first, second) = line.split_at(col as usize);
        lines.remove(row as usize);
        lines.insert(row as usize, String::from(second));
        lines.insert(row as usize, String::from(first));
        return Buffer { lines };
    }
}

#[derive(Debug)]
struct Cursor {
    row: i64,
    col: i64,
}

impl Cursor {
    fn down(&self, buffer: &Buffer) -> Cursor {
        return Cursor {
            row: self.row + 1,
            col: self.col,
        }.clamp(buffer)
    }

    fn up(&self, buffer: &Buffer) -> Cursor {
        return Cursor {
            row: self.row - 1,
            col: self.col,
        }.clamp(buffer)
    }

    fn left(&self, buffer: &Buffer) -> Cursor {
        return Cursor {
            row: self.row,
            col: self.col - 1,
        }.clamp(buffer)
    }

    fn right(&self, buffer: &Buffer) -> Cursor {
        return Cursor {
            row: self.row,
            col: self.col + 1,
        }.clamp(buffer)
    }

    fn clamp(&self, buffer: &Buffer) -> Cursor {
        let row =  std::cmp::min(self.row, buffer.line_count() -1);
        let row =  std::cmp::max(row, 0);

        let col =  std::cmp::min(self.col, buffer.line_length(row));
        let col =  std::cmp::max(col, 0);

        return Cursor {
            row,
            col,
        }
    }

    fn move_to_col(&self, col: i64) -> Cursor {
        return Cursor {
            row: self.row,
            col,
        }
    }
}

struct ANSI {
}

impl ANSI {
    fn clear_screen() {
        print!("[2J");
    }

    fn move_cursor(row: i64, column: i64) {
        print!("[{};{}H", row + 1, column + 1);
    }
}
