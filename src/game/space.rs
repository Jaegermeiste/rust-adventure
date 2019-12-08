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
use crate::game::backpack::*;

static	SPACE_DEFAULT_SPECIAL_ACTION_TEXT : &str =	"Turn Lights On";

pub enum SpaceType {
	None,
	GateHouse,
	ThroneRoom,
	Hallway,
	Bathroom,
	Armory,
	Bedroom,
	Dormitory,
	Dungeon,
	// NumTypes
}

pub struct SpaceData {
    pub right                       : Option<Rc<dyn Space>>,
    pub left                        : Option<Rc<dyn Space>>,
    pub top                         : Option<Rc<dyn Space>>,
    pub bottom                      : Option<Rc<dyn Space>>,

    pub x_coord                     : u32,
    pub y_coord                     : u32,

    pub special_action_text         : String,
    pub special_action_performed    : bool,

    pub pack                        : Backpack,

    pub movement_enabled            : bool,

    pub space_type                  : SpaceType,
}

impl Default for SpaceData {
    fn default() -> SpaceData {
        let data = SpaceData {
            right                       :   None,
            left                        :   None,
            top                         :   None,
            bottom                      :   None,
            x_coord                     :   0,
            y_coord                     :   0,
            special_action_text         :   SPACE_DEFAULT_SPECIAL_ACTION_TEXT.to_string(),
            special_action_performed    :   false,
            pack                        :   Backpack::new(),
            movement_enabled            :   true,
            space_type                  :   SpaceType::None,
        };
        return data;
    }
}

pub trait Space: GameObject {
    fn set_right                (&self, in_right : &Rc<dyn Space>);
    fn set_left                 (&self, in_right : &Rc<dyn Space>);
    fn set_top                  (&self, in_right : &Rc<dyn Space>);
    fn set_bottom               (&self, in_right : &Rc<dyn Space>);
    
    fn set_x_coord              (&self, coord : u32);
    fn set_y_coord              (&self, coord : u32);

    fn get_right                (&self)                             -> Rc<dyn Space>;
    fn get_left                 (&self)                             -> Rc<dyn Space>;
    fn get_top                  (&self)                             -> Rc<dyn Space>;
    fn get_bottom               (&self)                             -> Rc<dyn Space>;

    fn get_x_coord              (&self)                             -> u32;
    fn get_y_coord              (&self)                             -> u32;

    fn is_movement_enabled      (&self)                             -> bool;

    fn get_special_action_text  (&self)                             -> String;
    fn get_special_action_state (&self)                             -> bool;
    
    fn perform_special_action   (&self);

    fn get_space_type           (&self)                             -> SpaceType;

    fn get_pack                 (&self)                             -> Rc<Backpack>;
}