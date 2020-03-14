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
use traitcast::{traitcast};
use crate::game::gameobject::*;
use crate::game::locatable::*;
use crate::game::items::backpack::*;
use crate::game::items::other::junk::*;
use crate::game::world::spaces::space::*;

static  DUNGEON_NAME					: &str = "Dungeon";
static  DUNGEON_DOORS					: u32 = 2;
static  DUNGEON_FLAVOR_TEXT			    : &str = " The cell doors have long since rusted off their hinges and lie scattered and twisted upon the floor. You feel filled with a sense of foreboding.";
static  DUNGEON_SPECIAL_ACTION_TEXT 	: &str = "Investigate cell.";

pub struct Dungeon {
    // GameObject
    game_object_data    : GameObjectData,

    //Location
    location_data       : LocationData,

    // Space
    space_data          : SpaceData,
}

crate::impl_Space!(Dungeon);

impl  Default for Dungeon {
    fn default() -> Dungeon {
        let result = Dungeon {
            game_object_data    : GameObjectData    { name : String::from(DUNGEON_NAME), flavor_text : String::from(DUNGEON_FLAVOR_TEXT) },
            location_data       : LocationData      { ..Default::default()  },
            space_data          : SpaceData         { 
                                                        special_action_text : String::from(DUNGEON_SPECIAL_ACTION_TEXT),
                                                        movement_enabled    : true,
                                                        space_type          : SpaceType::Dungeon,
                                                        ..Default::default()
                                                    },
        };
        return result;
    }
}

impl Dungeon {
    fn perform_special_action(&mut self) {
        println!(" You uncover some {}.", JUNK_NAME);

        // Create a junk
        self.space_data.pack.borrow_mut().add_item(Rc::new(Junk::default()));
            
        self.space_data.special_action_performed = true;
    }
}