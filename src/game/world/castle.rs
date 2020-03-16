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
use std::cell::RefCell;
use crate::game::gameobject::*;
use crate::game::world::spaces::space::*;
use crate::game::world::spaces::gatehouse::*;
use crate::game::world::spaces::throneroom::*;

pub static CASTLE_NAME          : &str  = "Castle Wolfenstein";
pub static CASTLE_FLAVOR_TEXT   : &str  = " Heavily worn, this bag is made of a pliant leather of unknown origin.";

pub static CASTLE_WIDTH         : usize = 5;
pub static CASTLE_HEIGHT        : usize = 7;

pub struct Castle {
    // GameObject
    game_object_data    : GameObjectData,

    // Castle
    spaces      : Vec<Rc<RefCell<dyn Space>>>,

    start       : Option<Rc<RefCell<dyn Space>>>,
    finish      : Option<Rc<RefCell<dyn Space>>>,
}

impl Castle {
    pub fn new() -> Castle {
        let mut castle = Castle{ 
            game_object_data : GameObjectData::default(),
            spaces      : Vec::new(),
            start       : None,
            finish      : None
        };

        // Build the start
        castle.start = Some(Rc::new(RefCell::new(Gatehouse::default())));
        castle.start.as_ref().unwrap().borrow_mut().set_x_coord(CASTLE_WIDTH as u32 / 2);
        castle.start.as_ref().unwrap().borrow_mut().set_y_coord(0);
        castle.spaces.push(castle.start.as_ref().unwrap().clone());

        // Build the finish
        castle.finish = Some(Rc::new(RefCell::new(ThroneRoom::default())));
        castle.finish.as_ref().unwrap().borrow_mut().set_x_coord(CASTLE_WIDTH as u32 / 2);
        castle.finish.as_ref().unwrap().borrow_mut().set_y_coord(CASTLE_HEIGHT as u32 - 1);
        castle.spaces.push(castle.finish.as_ref().unwrap().clone());

        // Build the main hallway

        // Link the end of the hallway to the finish

        return castle;
    }

    pub fn get_start_space (&self) -> Option<Rc<RefCell<dyn Space>>> {
        return self.start.clone();
    }

    pub fn get_end_space (&self) -> Option<Rc<RefCell<dyn Space>>> {
        return self.finish.clone();
    }

    pub fn get_space_for_coordinates (&self, xCoord : u32, yCoord : u32) -> Option<Rc<RefCell<dyn Space>>> {
        let mut found : Option<Rc<RefCell<dyn Space>>> = None;

        for space in self.spaces.iter() {
            if (space.borrow().x_coord() == xCoord) && (space.borrow().y_coord() == yCoord) {
                found = Some(space.clone());
            }
        }

        return found;
    }
}

crate::impl_GameObject!(Castle);