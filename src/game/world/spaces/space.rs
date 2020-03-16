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
use crate::game::gameobject::*;
use crate::game::locatable::*;
use crate::game::items::backpack::*;
use crate::game::items::other::junk::*;

static	SPACE_DEFAULT_SPECIAL_ACTION_TEXT : &str =	"Light Torches";

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive)]
pub enum SpaceType {
	None = 0,
	GateHouse = 1,
	ThroneRoom = 2,
	Hallway = 3,
	Bathroom = 4,
	Armory = 5,
	Bedroom = 6,
	Dormitory = 7,
	Dungeon = 8,
	// NumTypes
}

pub struct SpaceData {
    pub right                       : Option<Rc<RefCell<dyn Space>>>,
    pub left                        : Option<Rc<RefCell<dyn Space>>>,
    pub top                         : Option<Rc<RefCell<dyn Space>>>,
    pub bottom                      : Option<Rc<RefCell<dyn Space>>>,

    pub special_action_text         : String,
    pub special_action_performed    : bool,

    pub pack                        : Rc<RefCell<Backpack>>,

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
            special_action_text         :   SPACE_DEFAULT_SPECIAL_ACTION_TEXT.to_string(),
            special_action_performed    :   false,
            pack                        :   Rc::new(RefCell::new(Backpack::new())),
            movement_enabled            :   true,
            space_type                  :   SpaceType::None,
        };

        let pointer_junk = Rc::new(Junk::default());
        data.pack.borrow_mut().add_item(pointer_junk);

        return data;
    }
}

pub trait Space: GameObject + Locatable {
    fn set_right                (&mut self, in_right : Rc<RefCell<dyn Space>>);
    fn set_left                 (&mut self, in_left : Rc<RefCell<dyn Space>>);
    fn set_top                  (&mut self, in_top : Rc<RefCell<dyn Space>>);
    fn set_bottom               (&mut self, in_bottom : Rc<RefCell<dyn Space>>);
    
    fn get_right                (&self)                             -> Option<Rc<RefCell<dyn Space>>>;
    fn get_left                 (&self)                             -> Option<Rc<RefCell<dyn Space>>>;
    fn get_top                  (&self)                             -> Option<Rc<RefCell<dyn Space>>>;
    fn get_bottom               (&self)                             -> Option<Rc<RefCell<dyn Space>>>;

    fn is_movement_enabled      (&self)                             -> bool;

    fn get_special_action_text  (&self)                             -> String;
    fn get_special_action_state (&self)                             -> bool;
    
    fn perform_special_action   (&self);

    fn get_space_type           (&self)                             -> SpaceType;

    fn get_pack                 (&mut self)                         -> Rc<RefCell<Backpack>>;
}

#[macro_export]
macro_rules! impl_Space { 
    ($T:ident) => {

        crate::impl_GameObject!($T);

        crate::impl_Locatable!($T);

        impl Space for $T {
            fn set_right(&mut self, in_right : Rc<RefCell<dyn Space>>) {
                self.space_data.right = Some(in_right); 
            }

            fn set_left(&mut self, in_left : Rc<RefCell<dyn Space>>) {
                self.space_data.left = Some(in_left); 
            }

            fn set_top(&mut self, in_top : Rc<RefCell<dyn Space>>) {
                self.space_data.top = Some(in_top); 
            }

            fn set_bottom(&mut self, in_bottom : Rc<RefCell<dyn Space>>) {
                self.space_data.bottom = Some(in_bottom); 
            }

            fn get_right(&self) -> Option<Rc<RefCell<dyn Space>>> {
                if self.space_data.right.is_some() {
                    return Some(self.space_data.right.clone().unwrap());
                }
                return None;
            }

            fn get_left(&self) -> Option<Rc<RefCell<dyn Space>>> {
                if self.space_data.left.is_some() {
                    return Some(self.space_data.left.clone().unwrap());
                }
                return None;
            }

            fn get_top(&self) -> Option<Rc<RefCell<dyn Space>>> {
                if self.space_data.top.is_some() {
                    return Some(self.space_data.top.clone().unwrap());
                }
                return None;
            }

            fn get_bottom(&self) -> Option<Rc<RefCell<dyn Space>>> {
                if self.space_data.bottom.is_some() {
                    return Some(self.space_data.bottom.clone().unwrap());
                }
                return None;
            }

            fn is_movement_enabled(&self) -> bool {
                return self.space_data.movement_enabled.clone();
            }

            fn get_special_action_text(&self) -> String {
                return self.space_data.special_action_text.clone();
            }

            fn get_special_action_state(&self) -> bool {
                return self.space_data.special_action_performed.clone();
            }

            // This needs to be implemented/overridden by derived types
            fn perform_special_action(&self) {
                ()      // DO NOTHING
            }

            fn get_space_type(&self) -> SpaceType {
                return self.space_data.space_type;
            }

            fn get_pack(&mut self) -> Rc<RefCell<Backpack>> {
                return self.space_data.pack.clone();
            }
        }

        // Type conversion
        traitcast!(struct $T: GameObject, Space);
    }
}