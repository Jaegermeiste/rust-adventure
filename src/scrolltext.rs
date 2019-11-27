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
extern crate crossterm;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    terminal::{Clear, ClearType, SetSize},
    input::{input, InputEvent, KeyEvent, MouseButton, MouseEvent},
    screen::{RawScreen, EnterAlternateScreen, LeaveAlternateScreen},
    queue,
    Result
};
extern crate textwrap;
use textwrap::Wrapper;
use crate::DEFAULT_TERMINAL_HEIGHT;
use crate::DEFAULT_TERMINAL_WIDTH;

pub struct ScrollText {
    scroll_pos : u32,
    max_pos : u32,
    console_height : u32,
    text : String,
}

impl  ScrollText {
    pub fn new() -> ScrollText {
        let scrolltext = ScrollText { 
            scroll_pos      : 0,
            max_pos         : 0,
            console_height  : DEFAULT_TERMINAL_HEIGHT as u32,
            text            : "".to_string(),
            };
        return scrolltext;
    }

    fn process_input_event(&mut self, key_event: InputEvent) -> bool {
        match key_event {
            InputEvent::Keyboard(k) => {
                match k {
                    KeyEvent::Char(c) => match c {
                        'q' => {
                            return true;
                        }
                        'Q' => {
                            return true;
                        }
                        ' ' => {
                            if self.scroll_pos < self.max_pos {
                                self.scroll_pos += self.console_height;
                            }
                            self.print_stored_text();
                        }
                        _ => {
                            ()
                        }
                    },
                    KeyEvent::Esc => {
                        return true;
                    }
                    KeyEvent::PageUp => {
                        if self.scroll_pos >= self.console_height {
                            self.scroll_pos -= self.console_height;
                        }
                        else {
                            self.scroll_pos = 0;
                        }
                        self.print_stored_text();
                    }
                    KeyEvent::PageDown => {
                        if self.scroll_pos < self.max_pos {
                            self.scroll_pos += self.console_height;
                        }
                        self.print_stored_text();
                    }
                    KeyEvent::Up => {
                        if self.scroll_pos > 0 {
                            self.scroll_pos -= 1;
                        }
                        self.print_stored_text();
                    }
                    KeyEvent::Down => {
                        if self.scroll_pos <= self.max_pos {
                            self.scroll_pos += 1;
                        }
                        self.print_stored_text();
                    }
                    _ => {
                        ()
                    }
                }
            }
            InputEvent::Mouse(m) => match m {
                MouseEvent::Press(b, _x, _y) => match b {
                    MouseButton::WheelUp => {
                        if self.scroll_pos > 0 {
                            self.scroll_pos -= 1;
                        }
                        self.print_stored_text();
                    },
                    MouseButton::WheelDown => {
                        if self.scroll_pos <= self.max_pos {
                            self.scroll_pos += 1;
                        }
                        self.print_stored_text();
                    },
                    _ => {
                        ()
                    }
                },
                _ => {
                    ()
                }
            },
            _ => (),
        }

        false
    }

    fn read_synchronously(&mut self) -> Result<()> {
        let _raw = RawScreen::into_raw_mode()?;

        let input = input();

        // enable mouse events to be captured.
        input.enable_mouse_mode()?;

        let mut sync_stdin = input.read_sync();

        loop {
            let event = sync_stdin.next();

            if let Some(key_event) = event {
                if self.process_input_event(key_event) {
                    break;
                }
            }
        }

        // disable mouse events to be captured.
        input.disable_mouse_mode()
    }

    fn print_stored_text(&mut self) {
        let mut stdout = stdout();
        let mut _result = queue!(stdout, Clear(ClearType::All));
        let mut _result2 = stdout.flush();

        let mut index = 0;
        for line in self.text.lines() {
            if index < (self.scroll_pos as usize) {
                // Do nothing
            }
            else if index >= ((self.scroll_pos + self.console_height - 1 - 1 /* Instruction line at bottom */) as usize) {
                // We're past the end
                break;
            }
            else {
                println!("{}", line);
            }

            index += 1;
        }

        println!("<▲>/<▼> arrows, <PgUp>/<PgDn>, or <Space> to navigate. <ESC> or <q> to return.");
    }

    pub fn print_text_range(&mut self, in_text : String, row : u32, con_height : u32) {
        // Format the incoming text
        let wrapper = Wrapper::with_termwidth();

        // Get us off the edge and process the line
        let mut formatted = wrapper.initial_indent(" ").subsequent_indent(" ").fill(in_text.as_str());

        // Pad the bottom of the output as neccesary
        while (formatted.lines().count() as u32) < (con_height + 1) {
            formatted.push_str("\n");
        }

        // Set internal variables
        self.text = formatted;
        let line_count = self.text.lines().count() as u32;
        self.max_pos = line_count - con_height;
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
        //_result = queue!(stdout, SetSize(DEFAULT_TERMINAL_WIDTH, con_height));
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