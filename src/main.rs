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

mod input;
mod menu;
mod menuaction;
mod menupage;
mod game;
use crate::input::*;
use crate::menu::*;
use crate::menuaction::*;
use crate::game::*;
use std::fs::File;
use std::io::{BufRead, BufReader, stdout, Write};
use std::rc::Rc;
extern crate crossterm;
use crossterm::{
    cursor::{Hide,  MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp, RestorePosition, SavePosition, Show},
    execute,
    terminal::{self, Clear, ClearType, ScrollDown, ScrollUp, SetSize},
    input::{input, InputEvent, KeyEvent, MouseButton, MouseEvent},
    screen::{RawScreen, AlternateScreen, EnterAlternateScreen, LeaveAlternateScreen},
    queue,
    Output,
    Result
};
extern crate textwrap;
use textwrap::fill;
use textwrap::wrap;
use textwrap::Wrapper;

static mut scroll_pos :u32 = 0;
const console_height : u32 = 24;

fn process_input_event(key_event: InputEvent) -> bool {
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
                        unsafe {
                            if (scroll_pos < std::u32::MAX) {
                                scroll_pos += console_height;
                            }
                            print_license_range(scroll_pos, console_height);
                        }
                    }
                    _ => {
                        ()
                    }
                },
                KeyEvent::Esc => {
                    return true;
                }
                KeyEvent::PageUp => {
                    //let _result = execute!(stdout(), ScrollDown(10));
                    unsafe {
                        if (scroll_pos >= console_height) {
                            scroll_pos -= console_height;
                        }
                        else {
                            scroll_pos = 0;
                        }
                        print_license_range(scroll_pos, console_height);
                    }
                }
                KeyEvent::PageDown => {
                    //let _result = execute!(stdout(), ScrollUp(10));
                    unsafe {
                        if (scroll_pos < std::u32::MAX) {
                            scroll_pos += console_height;
                        }
                        print_license_range(scroll_pos, console_height);
                    }
                }
                KeyEvent::Up => {
                    unsafe {
                        if (scroll_pos > 0) {
                            scroll_pos -= 1;
                        }
                        print_license_range(scroll_pos, console_height);
                    }
                }
                KeyEvent::Down => {
                    unsafe {
                        if (scroll_pos < std::u32::MAX) {
                            scroll_pos += 1;
                        }
                        print_license_range(scroll_pos, console_height);
                    }
                }
                _ => {
                    ()
                }
            }
        }
        InputEvent::Mouse(m) => match m {
            MouseEvent::Press(b, x, y) => match b {
                MouseButton::WheelUp => {
                    unsafe {
                        if (scroll_pos > 0) {
                            scroll_pos -= 1;
                        }
                        print_license_range(scroll_pos, console_height);
                    }
                },
                MouseButton::WheelDown => {
                    unsafe {
                        if (scroll_pos < std::u32::MAX) {
                            scroll_pos += 1;
                        }
                        print_license_range(scroll_pos, console_height);
                    }
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

fn read_synchronously() -> Result<()> {
    let _raw = RawScreen::into_raw_mode()?;

    let input = input();

    // enable mouse events to be captured.
    input.enable_mouse_mode()?;

    let mut sync_stdin = input.read_sync();

    loop {
        let event = sync_stdin.next();

        if let Some(key_event) = event {
            if process_input_event(key_event) {
                break;
            }
        }
    }

    // disable mouse events to be captured.
    input.disable_mouse_mode()
}

fn print_license_range(row : u32, con_height : u32) {
    let filename = "LICENSE.txt";
    //let file = File::open(filename).unwrap();
    //let reader = BufReader::new(file);
    let mut stdout = stdout();
    let mut _result = queue!(stdout, Clear(ClearType::All));
    stdout.flush();

    //let mut buf : Vec<u8> = Vec::new();
    //reader.get_mut().read_to_end(&buf);

    let license_text = std::fs::read_to_string(filename).unwrap();

    let wrapper = Wrapper::with_termwidth();

        // Get us off the edge and process the line
    let output = wrapper.initial_indent(" ").subsequent_indent(" ").fill(license_text.as_str());

    let mut index = 0;
    for line in output.lines() {
        //let line = line.unwrap();

        //let wrapper = Wrapper::with_termwidth();

        // Get us off the edge and process the line
        //let output = wrapper.initial_indent(" ").subsequent_indent(" ").fill(line.as_str());

        if index < (row as usize) {
            // Do nothing
        }
        else if index >= ((row + con_height - 1 - 1 /* Instruction line at bottom */) as usize) {
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

fn print_license() {
    let filename = "LICENSE.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();

        let output = fill(line.as_str(), 80);

        println!("{}", output);
 
        /*_result = queue!(
            stdout,
            Output(
                output,
            )
        );
        stdout.flush();*/
    }
}

fn show_license(input : &Rc<Input>) {


    // Open the file in read-only mode (ignoring errors).

    let mut stdout = stdout();
    let mut _result = queue!(stdout, EnterAlternateScreen);
    _result = queue!(stdout, Hide);
    _result = queue!(stdout, MoveTo(0, 400));
    _result = queue!(stdout, Clear(ClearType::FromCursorDown));
    _result = queue!(stdout, SetSize(80, 24));
    stdout.flush();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    print_license_range(0, 24);
    
    /*let mut key = input.get_keypress_from_console();

    while (key != 'q') || (key != 'Q')
    {
        if (key == 'w') || (key == 'W') {
            let _result = execute!(stdout(), ScrollUp(1));
        }
        else if (key == 's') || (key == 'S') {
            let _result = execute!(stdout(), ScrollDown(1));
        }
        key = input.get_keypress_from_console();
    }*/

    read_synchronously();

    _result = queue!(stdout, Show); // we must restore the cursor
    _result = queue!(stdout, LeaveAlternateScreen);
    stdout.flush();
}

fn main() {
    let input                       = Input::new();
    let mut menu_system             = Menu::new(&input);
    let main_menu_index             = menu_system.add_page("Main Menu");
    let about_page_index            = menu_system.add_page("About");
    let main_menu_new_game_option	= menu_system.add_action_to_page(main_menu_index,   MenuAction::new("Start New Game",	            MenuActionType::Selector));
    let main_menu_about_option	    = menu_system.add_action_to_page(main_menu_index,   MenuAction::new("About",	                    MenuActionType::Selector));
	let main_menu_quit_option		= menu_system.add_action_to_page(main_menu_index,   MenuAction::new("Exit Program",                 MenuActionType::ExitMenu));
    let about_menu_warranty_option	= menu_system.add_action_to_page(about_page_index,  MenuAction::new("Show Warranty Details",	    MenuActionType::Selector));
    let about_menu_redist_option	= menu_system.add_action_to_page(about_page_index,  MenuAction::new("Show Redistibution Details",   MenuActionType::Selector));
    let about_menu_license_option	= menu_system.add_action_to_page(about_page_index,  MenuAction::new("Show License",                 MenuActionType::Selector));
    let about_menu_main_menu_option	= menu_system.add_action_to_page(about_page_index,  MenuAction::new("Return to Main Menu",	        MenuActionType::ExitMenu));
    
    // License attribution
    println!(" rust-adventure (C) 2019 Jason George");
    println!(" This program comes with ABSOLUTELY NO WARRANTY; for details select 'About->Show Warranty Details' at the menu.");
    println!(" This is free software, and you are welcome to redistribute it under certain conditions; select 'About->Show Redistibution Details' at the menu for details.");

    // Display the Main Menu
	let mut main_menu_selection = menu_system.run_page(main_menu_index);

	// Loop so long as we haven't selected to quit
	while main_menu_selection != main_menu_quit_option {
		if main_menu_selection == main_menu_new_game_option	{
			let game = Game::new(&input);

			game.run();
        }
        else if main_menu_selection == main_menu_about_option {
            let mut about_menu_selection = menu_system.run_page(about_page_index);

            while about_menu_selection != about_menu_main_menu_option {     

                if about_menu_selection == about_menu_warranty_option {

                }
                else if about_menu_selection == about_menu_redist_option {

                }
                else if about_menu_selection == about_menu_license_option {
                    show_license(&input);
                }

                about_menu_selection = menu_system.run_page(about_page_index);
            }
        }
				
		// after running the function, return to the menu
		main_menu_selection = menu_system.run_page(main_menu_index);
	}
}
