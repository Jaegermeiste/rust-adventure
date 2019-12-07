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
use crate::game::gameobject::*;
use crate::game::space::*;

pub static CASTLE_NAME          : &str  = "Castle Wolfenstein";
pub static CASTLE_FLAVOR_TEXT   : &str  = " Heavily worn, this bag is made of a pliant leather of unknown origin.";

pub static CASTLE_WIDTH         : usize = 5;
pub static CASTLE_HEIGHT        : usize = 7;

pub struct Castle {
    // GameObject
    name        : String,
    flavor_text : String,

    // Castle
    spaces      : Vec<Rc<Space>>,
}

impl Castle {
    pub fn new() -> Castle {
        Castle{ 
            name        : CASTLE_NAME.to_string(),
            flavor_text : CASTLE_FLAVOR_TEXT.to_string(),
            spaces      : Vec::new(),
        }
    }
}

impl GameObject for Castle {
    fn name(&self) -> String {
        return self.name.clone();
    }

    fn flavor_text(&self) -> String {
        return self.flavor_text.clone();
    }
}