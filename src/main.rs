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

fn main() {
    let mut input           = Input::new();
    let menu_system         = Menu::new(&mut input);
    let main_menu_index     = menu_system.add_page("Main Menu");
    let about_page_index    = menu_system.add_page("About");
    let new_game_option	    = menu_system.add_action_to_page(main_menu_index, MenuAction::new("Start New Game",	MenuActionType::Selector));
    let about_option	    = menu_system.add_action_to_page(main_menu_index, MenuAction::new("About",	        MenuActionType::Selector));
	let quit_option		    = menu_system.add_action_to_page(main_menu_index, MenuAction::new("Exit Program",	MenuActionType::ExitMenu));

    // License attribution
    println!(" rust-adventure (C) 2019 Jason George");
    println!(" This program comes with ABSOLUTELY NO WARRANTY; for details select 'About->Show Warranty Details' at the menu.");
    println!(" This is free software, and you are welcome to redistribute it under certain conditions; select 'About->Show Redistibution Details' at the menu for details.");

    // Display the Main Menu
	let mut menu_selection = menu_system.run_page(main_menu_index);

	// Loop so long as we haven't selected to quit
	while menu_selection != quit_option
	{
		if menu_selection == new_game_option
		{
			let game = Game::new(&mut input);

			game.run();
        }
        else if menu_selection == about_option
        {
            menu_system.run_page(about_page_index);
        }
				
		// after running the function, return to the menu
		menu_selection = menu_system.run();
	}
}
