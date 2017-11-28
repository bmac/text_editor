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

    let editor = Editor::create(lines);
    
    editor.run();
}


#[derive(Debug)]
struct Editor {
    buffer: Buffer
}

impl Editor {
    fn run(&self) {
        let _stdout = std::io::stdout().into_raw_mode().unwrap();
        
        loop {
            self.render();
            if self.handle_input() {
                break
            }
        }
    }

    fn render(&self) {
        ANSI::clear_screen();
        ANSI::move_cursor(0, 0);
        self.buffer.render();
    }

    fn handle_input(&self) -> bool {
        let stdin = std::io::stdin();
        let key = stdin.keys().next().unwrap();

        let should_quit = match key {
            Ok(Key::Ctrl('c')) => true,
            Ok(Key::Ctrl('q')) => true,
            _ => false
        };
        
        println!("{:?}", key);

        return should_quit;
    }

    fn create(lines: Vec<String>) -> Editor {
        let buffer = Buffer { lines };

        Editor { buffer }
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
}

// #[derive(Debug)]
// struct Cursor {
// }

struct ANSI {
}

impl ANSI {
    fn clear_screen() {
        print!("[2J");
    }

    fn move_cursor(row: u64, column: u64) {
        print!("[{};{}H", row + 1, column + 1);
    }
}
