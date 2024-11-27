#![feature(portable_simd)]

use puyo::display::write_state;
use puyo::state::State;
use rand::{rng, Rng};
use std::io::Write;
use std::simd::u16x8;
use std::thread;
use termion::cursor;
use termion::cursor::DetectCursorPos;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

// Just for testing
fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let mut state = State::new();
    let mut rng = rng();
    for y in 0..13 {
        for x in 0..6  {
            state.set(rng.random_range(0..4), y, x);
        }
    }
    write!(stdout, "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n").unwrap();
    write!(stdout, "{}", cursor::Hide).unwrap();
    stdout.flush().unwrap();
    let (_, y) = stdout.cursor_pos().unwrap();
    write_state(&mut stdout, &state, y - 15).unwrap();
    for event in stdin.events() {
        if let Ok(Event::Key(Key::Ctrl('c'))) = event {
            write!(stdout, "{}", cursor::Show).unwrap();
            return;
        }
        if let Ok(Event::Key(Key::Char('i'))) = event {
            state = State::new();
            for y in 0..13 {
                for x in 0..6  {
                    state.set(rng.random_range(0..4), y, x);
                }
            }
            write_state(&mut stdout, &state, y - 15).unwrap();
        }
        if let Ok(Event::Key(Key::Char('c'))) = event {
            loop {
                let erase_mask = state.field[0].erase_mask() | state.field[1].erase_mask() | state.field[2].erase_mask() | state.field[3].erase_mask();
                if erase_mask == u16x8::splat(0) {
                    break;
                }
                let mut erase_state = State::new();
                for y in 0..13 {
                    for x in 0..6  {
                        if (erase_mask[x] >> y & 1) == 1 {
                            erase_state.set(4, y, x);
                        } else if let Some(c) = state.get(y, x) {
                            erase_state.set(c, y, x);
                        }
                    }
                }
                write_state(&mut stdout, &erase_state, y - 15).unwrap();
                thread::sleep(std::time::Duration::from_millis(200));
                state.erase_one();
                write_state(&mut stdout, &state, y - 15).unwrap();
                thread::sleep(std::time::Duration::from_millis(200));
            }
        }
    }
}
