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
use crate::game::items::other::goal::*;

static  THRONEROOM_NAME					: &str = "Throne Room";
static  THRONEROOM_DOORS				: u32 = 1;
static  THRONEROOM_FLAVOR_TEXT			: &str = " Two shiny spots appear in the floor, where the (supposedly) penitent petitioners of the king would kneel and beg for his favor.";
static  THRONEROOM_SPECIAL_ACTION_TEXT 	: &str = "Kneel in penitence.";

pub struct ThroneRoom {
    // GameObject
    game_object_data    : GameObjectData,

    //Location
    location_data       : LocationData,

    // Space
    space_data          : SpaceData,
}

crate::impl_Space!(ThroneRoom);

impl  Default for ThroneRoom {
    fn default() -> ThroneRoom {
        let result = ThroneRoom {
            game_object_data    : GameObjectData    { name : String::from(THRONEROOM_NAME), flavor_text : String::from(THRONEROOM_FLAVOR_TEXT) },
            location_data       : LocationData      { ..Default::default()  },
            space_data          : SpaceData         { 
                                                        special_action_text : String::from(THRONEROOM_SPECIAL_ACTION_TEXT),
                                                        movement_enabled    : false,
                                                        space_type          : SpaceType::ThroneRoom,
                                                        ..Default::default()
                                                    },
        };
        return result;
    }
}

impl ThroneRoom {
    fn perform_special_action(&mut self) {
        println!(" The fabled {} materializes on the throne.", GOAL_NAME);

        let pointer_goal = Rc::new(Goal::default());
        self.space_data.pack.borrow_mut().add_item(pointer_goal);
        
        self.space_data.special_action_performed = true;
    }
}