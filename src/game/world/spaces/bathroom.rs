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

static  BATHROOM_NAME					: &str = "Garderobe";
static  BATHROOM_DOORS					: u32 = 1;
static  BATHROOM_FLAVOR_TEXT			: &str = " It could definitely smell better in here.";
static  BATHROOM_SPECIAL_ACTION_TEXT 	: &str = "Light a candle.";

pub struct Bathroom {
    // GameObject
    game_object_data    : GameObjectData,

    //Location
    location_data       : LocationData,

    // Space
    space_data          : SpaceData,
}

crate::impl_Space!(Bathroom);

impl  Default for Bathroom {
    fn default() -> Bathroom {
        let result = Bathroom {
            game_object_data    : GameObjectData    { name : String::from(BATHROOM_NAME), flavor_text : String::from(BATHROOM_FLAVOR_TEXT) },
            location_data       : LocationData      { ..Default::default()  },
            space_data          : SpaceData         { 
                                                        special_action_text : String::from(BATHROOM_SPECIAL_ACTION_TEXT),
                                                        movement_enabled    : true,
                                                        space_type          : SpaceType::Bathroom,
                                                        ..Default::default()
                                                    },
        };
        return result;
    }
}

impl Bathroom {
    fn perform_special_action(&mut self) {
        println!(" With the candle lit, it smells slightly better in here, or at least the reek is masked.");
        
        self.space_data.special_action_performed = true;
    }
}