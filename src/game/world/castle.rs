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
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use rand::distributions::{Distribution, Uniform};
use crate::game::gameobject::*;
use crate::game::locatable::*;
use crate::game::world::spaces::space::*;
use crate::game::world::spaces::gatehouse::*;
use crate::game::world::spaces::throneroom::*;
use crate::game::world::spaces::hallway::*;
use crate::game::world::spaces::armory::*;
use crate::game::world::spaces::bedroom::*;
use crate::game::world::spaces::dormitory::*;
use crate::game::world::spaces::dungeon::*;
use crate::game::world::spaces::bathroom::*;

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

        // Build RNG
        let mut rng = rand::thread_rng();
        let space_type_roll = Uniform::from(SpaceType::Armory as u8 ..= SpaceType::Dungeon as u8);

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
        let mut working_last = castle.start.as_ref().unwrap().clone();
 
        for i in 1..(CASTLE_HEIGHT - 1) {
            // Create a new hallway segment
            let mut working_next = Rc::new(RefCell::new(Hallway::default()));
            castle.spaces.push(working_next.clone());

            // Set coordinate
            (*working_next.borrow_mut()).set_x_coord(CASTLE_WIDTH as u32 / 2);
            (*working_next.borrow_mut()).set_y_coord(i as u32);

            // Connect it to adjacent rooms
            (*working_last.borrow_mut()).set_top(working_next.clone());
            (*working_next.borrow_mut()).set_bottom(working_last.clone());
            
            // Create a left side room
            let mut working_side = match FromPrimitive::from_u8(space_type_roll.sample(&mut rng)) {
                Some(SpaceType::Armory)     => Rc::new(RefCell::new(Armory::default())) as Rc<RefCell<dyn Space>>,
                Some(SpaceType::Bedroom)    => Rc::new(RefCell::new(Bedroom::default())) as Rc<RefCell<dyn Space>>,
                Some(SpaceType::Dormitory)  => Rc::new(RefCell::new(Dormitory::default())) as Rc<RefCell<dyn Space>>,
                Some(SpaceType::Dungeon)    => Rc::new(RefCell::new(Dungeon::default())) as Rc<RefCell<dyn Space>>,
                None                        => Rc::new(RefCell::new(Dungeon::default())) as Rc<RefCell<dyn Space>>,
                _                           => Rc::new(RefCell::new(Dungeon::default())) as Rc<RefCell<dyn Space>>,
            };

            castle.spaces.push(working_side.clone());
            (*working_next.borrow_mut()).set_left(working_side.clone());
            (*working_side.borrow_mut()).set_right(working_next.clone());
            (*working_side.borrow_mut()).set_x_coord((*working_next.borrow_mut()).x_coord() - 1);
            (*working_side.borrow_mut()).set_y_coord(i as u32);

            // Terminate the left side room
            let mut working_bath = Rc::new(RefCell::new(Bathroom::default()));
            castle.spaces.push(working_bath.clone());
            (*working_side.borrow_mut()).set_left(working_bath.clone());
            (*working_bath.borrow_mut()).set_right(working_side.clone());
            (*working_bath.borrow_mut()).set_x_coord((*working_side.borrow_mut()).x_coord() - 1);
            (*working_bath.borrow_mut()).set_y_coord(i as u32);

            // Create a right side room
            working_side = match FromPrimitive::from_u8(space_type_roll.sample(&mut rng)) {
                Some(SpaceType::Armory)     => Rc::new(RefCell::new(Armory::default())) as Rc<RefCell<dyn Space>>,
                Some(SpaceType::Bedroom)    => Rc::new(RefCell::new(Bedroom::default())) as Rc<RefCell<dyn Space>>,
                Some(SpaceType::Dormitory)  => Rc::new(RefCell::new(Dormitory::default())) as Rc<RefCell<dyn Space>>,
                Some(SpaceType::Dungeon)    => Rc::new(RefCell::new(Dungeon::default())) as Rc<RefCell<dyn Space>>,
                None                        => Rc::new(RefCell::new(Dungeon::default())) as Rc<RefCell<dyn Space>>,
                _                           => Rc::new(RefCell::new(Dungeon::default())) as Rc<RefCell<dyn Space>>,
            };

            castle.spaces.push(working_side.clone());
            (*working_next.borrow_mut()).set_right(working_side.clone());
            (*working_side.borrow_mut()).set_left(working_next.clone());
            (*working_side.borrow_mut()).set_x_coord((*working_next.borrow_mut()).x_coord() + 1);
            (*working_side.borrow_mut()).set_y_coord(i as u32);

            // Terminate the right side room
            working_bath = Rc::new(RefCell::new(Bathroom::default()));
            castle.spaces.push(working_bath.clone());
            (*working_side.borrow_mut()).set_right(working_bath.clone());
            (*working_bath.borrow_mut()).set_left(working_side.clone());
            (*working_bath.borrow_mut()).set_x_coord((*working_side.borrow_mut()).x_coord() + 1);
            (*working_bath.borrow_mut()).set_y_coord(i as u32);

            // Increment the last
		    working_last = working_next;
        }

        // Link the end of the hallway to the finish
        (*working_last.borrow_mut()).set_top(castle.finish.as_ref().unwrap().clone());
        castle.finish.as_ref().unwrap().borrow_mut().set_bottom(working_last.clone());

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