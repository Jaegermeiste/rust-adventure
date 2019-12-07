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

pub static	SPACE_DEFAULT_SPECIAL_ACTION_TEXT : &str =	"Turn Lights On";

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
	NumTypes
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

    fn get_pack                 (&self);                            //-> Rc<Backpack>;
}