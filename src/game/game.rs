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
use crate::game::world::castle::*;
use crate::game::entities::player::*;

pub struct Game {
    input: Rc<Input>,

    castle: Castle,
    player: Player,
}

impl Game {
    pub fn new(in_input : &Rc<Input>) -> Game {
        Game { 
            input: Rc::clone(&in_input),
            castle: Castle::new(),
            player: Player::new(),
            }
    }
    
    pub fn run(&self) {}
}