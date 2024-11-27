use crate::state::State;
use std::io::{Result, Write};
use termion::color;
use termion::cursor;

static COLORS: [color::Rgb; 5] = [color::Rgb(251, 84, 88), color::Rgb(98, 146, 233), color::Rgb(253, 209, 98), color::Rgb(97, 191, 153), color::Rgb(203, 198, 204)];

pub fn write_state(output: &mut dyn Write, state: &State, term_offset_y: u16) -> Result<()> {
    write_frame(output, term_offset_y)?;
    for x in 0..6 {
        for y in 0..13 {
            if let Some(c) = state.get(y, x) {
                write_cell(output, y as i16, x as i16, COLORS[c as usize], term_offset_y)?;
            }
        }
    }
    // if let Some(PairPuyo { axis, child }) = state.next {
    //     write!(output, "{}{}  {}", cursor::Goto(16, term_offset_y + 1), color::Bg(COLORS[child as usize]), color::Bg(color::Reset))?;
    //     write!(output, "{}{}  {}", cursor::Goto(16, term_offset_y + 2), color::Bg(COLORS[axis as usize]), color::Bg(color::Reset))?;
    // }
    // if let Some(PairPuyo { axis, child }) = state.next_next {
    //     write!(output, "{}{}  {}", cursor::Goto(16, term_offset_y + 4), color::Bg(COLORS[child as usize]), color::Bg(color::Reset))?;
    //     write!(output, "{}{}  {}", cursor::Goto(16, term_offset_y + 5), color::Bg(COLORS[axis as usize]), color::Bg(color::Reset))?;
    // }
    write!(output, "{}", cursor::Goto(1, term_offset_y + 15))?;
    output.flush()?;
    Ok(())
}

fn write_frame(output: &mut dyn Write, term_offset_y: u16) -> Result<()> {
    let frame_color = color::Rgb(250, 250, 250);
    let x_color = color::Rgb(251, 84, 88);
    for i in 0..13 {
        let y = (i + term_offset_y as i16 + 1) as u16;
        if i == 0 {
            write!(output, "{}{}              ", cursor::Goto(1, y), color::Bg(color::Reset))?;
        } else {
            write!(output, "{}{} {}            {} ", cursor::Goto(1, y), color::Bg(frame_color), color::Bg(color::Reset), color::Bg(frame_color))?;
        }
    }
    write!(output, "{}              {}", cursor::Goto(1, term_offset_y + 14), color::Bg(color::Reset))?;
    write!(output, "{}{}XX", cursor::Goto(6, term_offset_y + 2), color::Fg(x_color))?;
    Ok(())
}

fn write_cell(output: &mut dyn Write, y: i16, x: i16, color: color::Rgb, term_offset_y: u16) -> Result<()> {
    let x = (x * 2 + 2) as u16;
    let y = (term_offset_y as i16 + 12 - y + 1) as u16;
    write!(output, "{}{}  {}", cursor::Goto(x, y), color::Bg(color), color::Bg(color::Reset))
}
