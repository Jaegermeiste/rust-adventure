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

pub struct Menu {
    pub input: *const Input,
}

impl Menu {
    pub fn new(in_input : &Input) -> Menu {
        Menu { 
            input: in_input,
            }
    }

    pub fn add_page(&self, in_name : &str) -> u32 {(0)}
    
    pub fn add_action_to_page(&self, in_page_index : u32, in_action : MenuAction) -> u32 {(0)}
    
    pub fn run_page(&self, in_page_index : u32) -> u32 {(0)}
    
    pub fn run(&self) -> u32 {(0)}
}