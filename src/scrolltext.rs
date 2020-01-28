/**************************************************************************
 
    rust-adventure - a small adventure game written in Rust
    Copyright (C) 2019 Jason George

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.

**************************************************************************/
use std::io::{stdout, Write};
use std::rc::Rc;
extern crate crossterm;
use crossterm::{
    execute,
    cursor::{Hide, MoveTo, Show},
    terminal::{self, Clear, ClearType, SetSize, EnterAlternateScreen, LeaveAlternateScreen},
    event::{Event, KeyCode, MouseEvent},
    style::{Color, SetForegroundColor, SetBackgroundColor, ResetColor, Print},
    queue,
    Result
};
extern crate textwrap;
use textwrap::Wrapper;
use crate::DEFAULT_TERMINAL_HEIGHT;
use crate::DEFAULT_TERMINAL_WIDTH;
use crate::input::*;

pub struct ScrollText {
    scroll_pos : u32,
    max_pos : u32,
    console_height : u32,
    text : String,
    input: Rc<Input>,
}

impl  ScrollText {
    pub fn new(in_input : &Rc<Input>) -> ScrollText {
        let scrolltext = ScrollText { 
            scroll_pos      : 0,
            max_pos         : 0,
            console_height  : DEFAULT_TERMINAL_HEIGHT as u32,
            text            : "".to_string(),
            input           : Rc::clone(&in_input),
            };
        return scrolltext;
    }

    fn scroll(&mut self, lines : i32) {
        let mut adjustment = lines;
        if (lines < 0) && (lines.abs() > (self.scroll_pos as i32)) {
            adjustment = -(self.scroll_pos as i32);
        }

        if lines > (self.max_pos as i32) {
            adjustment = self.max_pos as i32;
        }

        let target_pos : i32 = (self.scroll_pos as i32) + adjustment;
        
        if target_pos < 0 {
            self.scroll_pos = 0;
        }
        else if target_pos > (self.max_pos as i32) {
            self.scroll_pos = self.max_pos;
        }
        else {
            self.scroll_pos = target_pos as u32;
        }

        self.print_stored_text();
    }

    fn process_input_event(&mut self, event: Event) -> bool {
        match event {
            Event::Key(k) => {
                match k.code {
                    KeyCode::Char(c) => match c {
                        'q' => {
                            return true;
                        }
                        'Q' => {
                            return true;
                        }
                        ' ' => {
                            self.scroll(self.console_height as i32 - 2);
                        }
                        _ => {
                            ()
                        }
                    },
                    KeyCode::Esc => {
                        return true;
                    }
                    KeyCode::PageUp => {
                        self.scroll(-(self.console_height as i32 - 2));
                    }
                    KeyCode::PageDown => {
                        self.scroll(self.console_height as i32 - 2);
                    }
                    KeyCode::Up => {
                        self.scroll(-1);
                    }
                    KeyCode::Down => {
                        self.scroll(1);
                    }
                    _ => (),
                }
            },
            Event::Mouse(m) => match m {
                MouseEvent::ScrollUp(_col, _row, _modifier) => {
                        self.scroll(-1);
                    },
                MouseEvent::ScrollDown(_col, _row, _modifier) => {
                        self.scroll(1);
                    },
                _ => (),
            },
            _ => (),
        }

        false
    }

    fn read_synchronously(&mut self) -> Result<()> {
        let _ = terminal::enable_raw_mode();

        // enable mouse events to be captured.
        let _ = self.input.enable_mouse_mode();

        //let mut sync_stdin = self.input.read_sync();

        loop {
            //let event = sync_stdin.next();
            let event = self.input.read_sync();

            if let Some(key_event) = event.ok() {
                if self.process_input_event(key_event) {
                    break;
                }
            }
        }

        // disable mouse events to be captured.
        let _ = self.input.disable_mouse_mode();

        terminal::disable_raw_mode()
    }

    fn print_stored_text(&mut self) {
        let mut stdout = stdout();
        let mut _result = queue!(stdout, Clear(ClearType::All));
        let mut _result2 = stdout.flush();

        let mut index = 0;
        for line in self.text.lines() {
            /*if index < (self.scroll_pos as usize) {
                // Do nothing
            }
            else*/ if index >= ((self.scroll_pos + self.console_height - 2) as usize) {
                // We're past the end
                break;
            }
            else {
                println!("{}", line);
            }

            index += 1;
        }

        //println!("{}/{} <▲>/<▼>, <PgUp>/<PgDn>, or <Space> to navigate. <ESC> or <q> to return.", self.scroll_pos, self.max_pos);
        let nav_bar = format!(" {:03}/{:03} <▲>/<▼>, <PgUp>/<PgDn>, or <Space> to navigate. <ESC> or <q> to return ", self.scroll_pos, self.max_pos);
        _result = execute!(
            stdout,
            SetForegroundColor(Color::Black),
            SetBackgroundColor(Color::White),
            Print(nav_bar),
            ResetColor
        );
    }

    pub fn print_text_range(&mut self, in_text : String, row : u32, con_height : u32) {
        // Format the incoming text
        let wrapper = Wrapper::with_termwidth();

        // Get us off the edge and process the line
        let mut formatted = wrapper.initial_indent(" ").subsequent_indent(" ").fill(in_text.as_str());

        // Pad the bottom of the output as neccesary
        while (formatted.lines().count() as u32) < (con_height - 2) {
            formatted.push_str("\n");
        }

        // Set internal variables
        self.text = formatted;
        let line_count = self.text.lines().count() as u32;
        if line_count < con_height {
            self.max_pos = 0;       // All text fits on one screen
        }
        else {
            self.max_pos = std::cmp::max(line_count, con_height);
        }
        self.console_height = con_height;
        if row <= self.max_pos {
            self.scroll_pos = row;
        }

        else {
            self.scroll_pos = self.max_pos;
        }

        let mut stdout = stdout();
        let mut _result = queue!(stdout, EnterAlternateScreen);
        _result = queue!(stdout, Hide);
        _result = queue!(stdout, MoveTo(0, con_height as u16));
        _result = queue!(stdout, Clear(ClearType::FromCursorDown));
        _result = queue!(stdout, SetSize(DEFAULT_TERMINAL_WIDTH, con_height as u16));
        let mut _result2 = stdout.flush();

        self.print_stored_text();

        let _res = self.read_synchronously();

        _result = queue!(stdout, Show); // we must restore the cursor
        _result = queue!(stdout, LeaveAlternateScreen);
        _result2 = stdout.flush();
    }

    pub fn print_file_range(&mut self, filename : &str, row : u32, con_height : u32) {
        let temp_text = std::fs::read_to_string(filename).unwrap();

        self.print_text_range(temp_text, row, con_height);
    }

}