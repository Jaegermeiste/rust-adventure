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
use std::io::{Write};
use std::fmt;
use crate::input::*;
use crate::menuaction::*;

pub const MAX_MENU_PAGE_OPTIONS : usize =	1024;

pub struct MenuPage {
    page_title: String,
    menu_items: Vec<MenuAction>,
    menu_number: u32,
}

impl MenuPage {
    pub fn new(in_title : &str, in_index : u32) -> MenuPage {
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
    
    pub fn select_action(&mut self, selection : u32, input : &Input) -> (MenuActionType, i32) {
        let mut return_val: (MenuActionType, i32) = (MenuActionType::Null, 0);
        let mut input_value: i32 = 0;
        let mut upper_limit: i32 = 0;
        let mut allowed_value_list: Vec<i32>;
        let mut input_string: String = "".to_string();

        if (selection >= 0) && (selection <= (self.menu_items.len() as u32)) {
            // Run the action
            return_val = self.menu_items.get(selection as usize).unwrap().on_selected();

            if return_val.0 >= MenuActionType::Bool {
                // Output a white space
                println!("");

                // Output the title
                println!("{}", self.menu_items.get(selection as usize).unwrap().get_string());

                if (self.menu_items.get(selection as usize).unwrap().get_type() == MenuActionType::String) ||
                    (self.menu_items.get(selection as usize).unwrap().get_type() == MenuActionType::StringListOverride) {
                    // Action has a meaningful value
                    println!(" Current value is {}, default value is {}. ", self.menu_items.get(selection as usize).unwrap().get_value_string() ,self.menu_items.get(selection as usize).unwrap().get_default_value_string());
                }
                else {
                    // Action has a meaningful value
                    println!(" Current value is {}, default value is {}. ", self.menu_items.get(selection as usize).unwrap().get_value_int() ,self.menu_items.get(selection as usize).unwrap().get_default_value());
                }

                upper_limit = self.menu_items.get(selection as usize).unwrap().get_max_range_value();

                if self.menu_items.get(selection as usize).unwrap().get_type() == MenuActionType::IntRangeUpperInclusive {
                    // Action has a range defined
                    println!(" Valid range is an integer from  {} to {}, inclusive. ", self.menu_items.get(selection as usize).unwrap().get_min_range_value(), upper_limit);
                }
                else if self.menu_items.get(selection as usize).unwrap().get_type() == MenuActionType::IntRangeUpperExclusive {
                    // Action has a range defined
                    upper_limit = self.menu_items.get(selection as usize).unwrap().get_max_range_value() - 1;	// Since range is exclusive, subtract 1
                    println!(" Valid range is an integer from  {} to {}, inclusive. ", self.menu_items.get(selection as usize).unwrap().get_min_range_value(), upper_limit);
                }
                else if self.menu_items.get(selection as usize).unwrap().get_type() == MenuActionType::IntList {
                    // Action has a valid value list defined
                    allowed_value_list = self.menu_items.get(selection as usize).unwrap().get_allowed_value_list();
                    print!(" Valid values are any of the following: ");
                    
                    let mut index: usize = 0;
                    for element in allowed_value_list.iter() {
                        if index < (allowed_value_list.len() - 1) {
                            // We haven't hit the end of the list yet, so output a comma and space
                            print!("{}, ", element);
                        }
                        else {
                            // End of the list, output a period and newline
                            println!("{}.", element);
                        }

                        index += 1;
                    }
                }
                else if self.menu_items.get(selection as usize).unwrap().get_type() == MenuActionType::StringList {
                    // Action has a valid value list defined
                    allowed_value_list = self.menu_items.get(selection as usize).unwrap().get_allowed_value_list();
                    let string_list = self.menu_items.get(selection as usize).unwrap().get_allowed_value_strings();

                    print!(" Valid values are any of the following: ");
                    
                    let mut index: usize = 0;
                    for element in allowed_value_list.iter() {
                        if index < (allowed_value_list.len() - 1) {
                            // We haven't hit the end of the list yet, so output a comma and space
                            print!("{}) {}, ", element, string_list.get(index).unwrap());
                        }
                        else {
                            // End of the list, output a period and newline
                            println!("{}) {}.", element, string_list.get(index).unwrap());
                        }

                        index += 1;
                    }
                }
                else if self.menu_items.get(selection as usize).unwrap().get_type() == MenuActionType::String {
                    // Action can contain any string
                    println!(" Valid value is any string. ");
                }

                // Query the user for input
                println!("");
                if (self.menu_items.get(selection as usize).unwrap().get_type() == MenuActionType::IntList) || 
                    (self.menu_items.get(selection as usize).unwrap().get_type() == MenuActionType::StringList) {
                    // Action has a valid value list defined
                    allowed_value_list = self.menu_items.get(selection as usize).unwrap().get_allowed_value_list();
                    print!(" Please enter one of the following: ");

                    let mut index: usize = 0;
                    for element in allowed_value_list.iter() {
                        if index < (allowed_value_list.len() - 1) {
                            // We haven't hit the end of the list yet, so output a comma and space
                            print!("{}, ", element);
                        }
                        else {
                            // End of the list, output a colon
                            print!("{}: ", element);
                        }

                        index += 1;
                    }

                    // Obtain the input from our validation function
                    input_value = input.get_signed_integer_value_from_console_list(allowed_value_list);
                }
                else if (self.menu_items.get(selection as usize).unwrap().get_type() == MenuActionType::String) || 
                        (self.menu_items.get(selection as usize).unwrap().get_type() == MenuActionType::StringListOverride) {
                            print!(" Please enter a string: ");

                    // Obtain the input from our validation function
                    input_string = input.get_string_from_console();
                }
                else {
                    print!(" Please enter an integer ranging from {} to {}: ", self.menu_items.get(selection as usize).unwrap().get_min_range_value(), upper_limit);

                    // Obtain the input from our validation function
                    input_value = input.get_signed_integer_value_from_console_range(self.menu_items.get(selection as usize).unwrap().get_min_range_value(), upper_limit, DEFAULT_CANCEL_VAL);
                }

                if (self.menu_items.get(selection as usize).unwrap().get_type() == MenuActionType::String) || 
                    (self.menu_items.get(selection as usize).unwrap().get_type() == MenuActionType::StringListOverride) {
                    // Pass the valid input to the action for storage
                    self.menu_items.get_mut(selection as usize).unwrap().set_value_string(input_string);
                }
                else {
                    // Pass the valid input to the action for storage
                    self.menu_items.get(selection as usize).unwrap().set_value_i32(input_value);
                }
            }  
        }
        else {
            println!(" ERROR: Value {} provided to MenuPage::select_action is outside the legal interval 1 - {}, inclusive. Selection Failed.", selection, self.menu_items.len());
        }

        return return_val;
    }

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
        let _result = std::io::stdout().flush();
    }

    pub fn clear (&mut self) {
        // Empty the vector
        self.menu_items.truncate(0);
    }
}

impl fmt::Display for MenuPage {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        self.draw();
        let result: Result<(), std::fmt::Error> = Ok(());
        return result;
    }
}