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

static  BEDROOM_NAME					: &str = "Royal Bedchamber";
static  BEDROOM_DOORS					: u32 = 2;
static  BEDROOM_FLAVOR_TEXT			    : &str = " It is clear that this room used to be opulent and comfortable. All that remains now is rat-eaten tatters.";
static  BEDROOM_SPECIAL_ACTION_TEXT 	: &str = "Open Wardrobe";

pub struct Bedroom {
    // GameObject
    game_object_data    : GameObjectData,

    //Location
    location_data       : LocationData,

    // Space
    space_data          : SpaceData,
}

crate::impl_Space!(Bedroom);

impl  Default for Bedroom {
    fn default() -> Bedroom {
        let result = Bedroom {
            game_object_data    : GameObjectData    { name : String::from(BEDROOM_NAME), flavor_text : String::from(BEDROOM_FLAVOR_TEXT) },
            location_data       : LocationData      { ..Default::default()  },
            space_data          : SpaceData         { 
                                                        special_action_text : String::from(BEDROOM_SPECIAL_ACTION_TEXT),
                                                        movement_enabled    : true,
                                                        space_type          : SpaceType::Bedroom,
                                                        ..Default::default()
                                                    },
        };
        return result;
    }
}

impl Bedroom {
    fn perform_special_action(&mut self) {
        println!(" Some {} falls out.", JUNK_NAME);

        // Create a junk
        self.space_data.pack.borrow_mut().add_item(Rc::new(Junk::default()));
            
        self.space_data.special_action_performed = true;
    }
}