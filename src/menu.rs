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
use std::rc::Rc;
use crate::input::*;
use crate::menuaction::*;
use crate::menupage::*;

pub struct Menu {
    menu_pages: Vec<MenuPage>,
    input: Rc<Input>,
    current_menu_index: u32,
}

impl Menu {
    pub fn new(in_input : &Rc<Input>) -> Menu {
        Menu { 
            menu_pages: Vec::new(),
            input: Rc::clone(&in_input),
            current_menu_index: 0,
            }
    }

    pub fn add_page(&mut self, in_name : &str) -> u32 {
        self.menu_pages.push(MenuPage::new(in_name, self.menu_pages.len() as u32));
        // Modulate the returned index by MAX_MENU_PAGE_OPTIONS to ensure unique indexes per option
        return ((self.menu_pages.len() - 1) * MAX_MENU_PAGE_OPTIONS) as u32;
    }
    
    pub fn add_action_to_page(&mut self, in_page_index : u32, in_action : MenuAction) -> u32 {
        let mut action_index : u32  = std::u32::MAX;
        let current_menu_index = (in_page_index as usize) / MAX_MENU_PAGE_OPTIONS;

        if let Some(ref _page) = Some(self.menu_pages.get(current_menu_index)) {
            action_index = self.menu_pages.get_mut(current_menu_index).unwrap().add_item(in_action);
            action_index += (current_menu_index * MAX_MENU_PAGE_OPTIONS) as u32;
        }

        return action_index;
    }

    pub fn clear_page(&mut self, in_page_index: u32) {
        let adjusted_page_index = (in_page_index as usize) / MAX_MENU_PAGE_OPTIONS;
        self.menu_pages.get_mut(adjusted_page_index).unwrap().clear();
    }

    pub fn set_defaults(&self) {
        for page in self.menu_pages.iter() {
            page.set_defaults();
        }
    }
    
    pub fn run_page(&mut self, in_page_index : u32) -> u32 {
        self.current_menu_index = in_page_index.clone();

        return self.run();
    }
    
    pub fn run(&mut self) -> u32 {
        let mut selection = 0;
        let mut return_val : u32 = std::u32::MAX;
        let mut action_val = (MenuActionType::Null, -1);
        let mut	run_loop = true;

        while (self.menu_pages.len() > 0) && (run_loop == true)
        {
            // Draw the current menu
            //print!("{}", self.menu_pages.get(self.current_menu_index as usize).unwrap());
            self.menu_pages.get(self.current_menu_index as usize).unwrap().draw();

            // Get the selection value from the input validator
            selection = self.input.get_unsigned_integer_value_from_console_range(1, self.menu_pages.get(self.current_menu_index as usize).unwrap().get_num_items(), DEFAULT_CANCEL_VAL);

            // Compensate for zero indexing shift (i.e. option 1 is really index 0)
            selection -= 1;

            // Touch the selected MenuAction
            action_val = self.menu_pages.get_mut(self.current_menu_index as usize).unwrap().select_action(selection, &self.input);

            // Handle the output of the MenuAction as necessary
            match action_val.0 {
                MenuActionType::LaunchMenuPage => {
                self.current_menu_index = action_val.1 as u32;
                self.current_menu_index /= MAX_MENU_PAGE_OPTIONS as u32;
                    return_val = (self.current_menu_index * (MAX_MENU_PAGE_OPTIONS as u32)) + selection;
                },

                MenuActionType::Selector => {
                    // We have a menu selection, so break out of the loop
                    run_loop = false;

                    return_val = (self.current_menu_index * (MAX_MENU_PAGE_OPTIONS as u32)) + selection;
                },

                MenuActionType::ExitMenu => {
                    // We have a menu selection, so break out of the loop
                    run_loop = false;

                    return_val = (self.current_menu_index * (MAX_MENU_PAGE_OPTIONS as u32)) + selection;
                },

                _ => {
                    // Give the outside system a chance to handle the selection
                    run_loop = false;
                    return_val = (self.current_menu_index * (MAX_MENU_PAGE_OPTIONS as u32)) + selection;
                }
            }
        }

        return return_val;
    }
}