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
use std::io::Write;
use crate::input::Input;
use crate::menuaction::MenuAction;
use crate::menuaction::MenuActionType;

const MAX_MENU_PAGE_OPTIONS : usize =	1024;

pub struct MenuPage {
    page_title: String,
    menu_items: Vec<MenuAction>,
    menu_number: u32,
}

impl MenuPage {
    pub fn new(&self, in_title : &str, in_index : u32) -> MenuPage {
        MenuPage { 
            page_title: in_title.to_string(),
            menu_items: Vec::new(),
            menu_number: in_index,
            }
    }

    pub fn get_num_items(&self) -> u32 {(self.menu_items.len() as u32)}
    
    pub fn add_item(&mut self, in_action : MenuAction) -> u32 {
        if self.menu_items.len() < MAX_MENU_PAGE_OPTIONS {
            self.menu_items.push(in_action);
        }
        else {
            println!(" ERROR: Attempt to add more than {} MenuActions to a MenuPage.", MAX_MENU_PAGE_OPTIONS);
        }
        return (self.menu_items.len() - 1) as u32;
    }
    
    pub fn set_defaults(&self) {
        for item in self.menu_items.iter() {
            item.reset();
        }
    }
    
    pub fn select_action(&self, selection : u32, input : Input) -> (MenuActionType, i32) {(MenuActionType::Null, 0)}

    pub fn draw(&self) {
        // Output the menu page title
        println!("\n {}\n", self.page_title);

        // Loop though all actions in the MenuPage
        let mut counter : u32 = 0;
        for item in self.menu_items.iter() {
            // counter + 1 because we don't want zero indexed menu options
            if item.get_type() >= MenuActionType::StringList {
                // Action is a settable string parameter
                println!("  {}.  Current: {}, Default: {}. ", (counter + 1), item.get_value_string(), item.get_default_value_string());
            }
            else if item.get_type() >= MenuActionType::Bool {
                // Action is a settable numeric parameter
                println!("  {}.  Current: {}, Default: {}. ", (counter + 1), item.get_value_int(), item.get_default_value());
            }
            else {
                // Action is not settable
                println!("  {}.  {}. ", (counter + 1), item.get_string());
            }

            counter += 1;
        }

        // No line return
        print!("\n Please make a menu selection from 1 to {}: ", self.menu_items.len());
        std::io::stdout().flush();
    }

    pub fn clear (&mut self) {
        // Empty the vector
        self.menu_items.truncate(0);
    }
}