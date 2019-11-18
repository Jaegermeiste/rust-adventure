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
use crate::input::Input;
use crate::menuaction::MenuAction;

const MAX_MENU_PAGE_OPTIONS : u32 =	1024;

pub struct MenuPage {
    page_title: String,
    menu_items: Vec<MenuAction>,
    menu_number: u32,
}

impl MenuPage {
    pub fn new(&self, in_title : &str, in_index u32) -> MenuPage {
        MenuPage { 
            page_title: in_title,
            menu_number: in_index,
            }
    }

    pub fn get_num_items(&self) -> u32 {(menu_items.len())}
    
    pub fn add_item(&self, in_action : MenuAction) -> u32 {
        if menu_items.len() < MAX_MENU_PAGE_OPTIONS
        {
            menu_items.push(in_action);
        }
        else
        {
            println!(" ERROR: Attempt to add more than {} MenuActions to a MenuPage.", MAX_MENU_PAGE_OPTIONS);
        }
        return (menu_items.len() - 1);
    }
    
    pub fn run_page(&self, in_page_index : u32) -> u32 {(0)}
    
    pub fn run(&self) -> u32 {(0)}
}