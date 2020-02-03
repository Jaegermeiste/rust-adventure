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

static  GATEHOUSE_NAME					: &str = "Barbican";
static  GATEHOUSE_DOORS					: u32 = 1;
static  GATEHOUSE_FLAVOR_TEXT			: &str = " The entryway to the castle was once imposing and majestic. Now, it is covered in vines and bird droppings.";
static  GATEHOUSE_SPECIAL_ACTION_TEXT	: &str = "Open Portcullis";

pub struct Gatehouse {
    // GameObject
    game_object_data    : GameObjectData,

    //Location
    location_data       : LocationData,

    // Space
    space_data          : SpaceData,
}

crate::impl_Space!(Gatehouse);

impl  Default for Gatehouse {
    fn default() -> Gatehouse {
        let result = Gatehouse {
            game_object_data    : GameObjectData    { name : String::from(GATEHOUSE_NAME), flavor_text : String::from(GATEHOUSE_FLAVOR_TEXT) },
            location_data       : LocationData      { ..Default::default()  },
            space_data          : SpaceData         { 
                                                        special_action_text : String::from(GATEHOUSE_SPECIAL_ACTION_TEXT),
                                                        movement_enabled    : false,
                                                        space_type          : SpaceType::GateHouse,
                                                        ..Default::default()
                                                    },
        };
        return result;
    }
}

impl Gatehouse {
    fn perform_special_action(&mut self) {
        println!(" Portcullis is now open. You can now move.");
        
        self.space_data.special_action_performed = true;
        self.space_data.movement_enabled = true;

    }
}