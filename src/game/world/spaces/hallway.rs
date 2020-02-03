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
use crate::game::world::spaces::space::*;

static  HALLWAY_NAME					: &str = "Hallway";
static  HALLWAY_DOORS					: u32 = 4;
static  HALLWAY_FLAVOR_TEXT			    : &str = " Rusty and blemished suits of armor, many missing pieces, line the walls.";
static  HALLWAY_SPECIAL_ACTION_TEXT 	: &str = "Light Torches";

pub struct Hallway {
    // GameObject
    game_object_data    : GameObjectData,

    //Location
    location_data       : LocationData,

    // Space
    space_data          : SpaceData,
}

crate::impl_Space!(Hallway);

impl  Default for Hallway {
    fn default() -> Hallway {
        let result = Hallway {
            game_object_data    : GameObjectData    { name : String::from(HALLWAY_NAME), flavor_text : String::from(HALLWAY_FLAVOR_TEXT) },
            location_data       : LocationData      { ..Default::default()  },
            space_data          : SpaceData         { 
                                                        special_action_text : String::from(HALLWAY_SPECIAL_ACTION_TEXT),
                                                        movement_enabled    : false,
                                                        space_type          : SpaceType::Hallway,
                                                        ..Default::default()
                                                    },
        };
        return result;
    }
}

impl Hallway {
    fn perform_special_action(&mut self) {
        println!(" Torches are now lit. You can now see and move.");
        
        self.space_data.special_action_performed = true;
        self.space_data.movement_enabled = true;

    }
}