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
use traitcast::{TraitcastFrom};
use crate::game::gameobject::*;

#[derive(PartialOrd, PartialEq, Eq, Copy, Clone, Debug)]
pub enum ItemProperty {
	None,
	Droppable,
	Max
}

#[derive(PartialOrd, PartialEq, Eq, Copy, Clone, Debug)]
pub enum ItemType {
	Default,
	Weapon,
	Shield,
	Health,
	ThroneRoomKey,
	GameWin,
	Max
}

pub static	DEFAULT_ITEM_WEIGHT : u32 =	1;

pub struct ItemData {
    pub weight      : u32,
    pub property    : ItemProperty,
    pub item_type   : ItemType,
}

impl Default for ItemData {
    fn default() -> ItemData {
        let data = ItemData {
            weight      :   DEFAULT_ITEM_WEIGHT,
            property    :   ItemProperty::None,
            item_type   :   ItemType::Default,
        };
        return data;
    }
}

pub trait Item: GameObject + TraitcastFrom {
    fn  get_item_weight         (&self) -> u32;
    fn  get_item_property       (&self) -> ItemProperty;
    fn  get_item_type           (&self) -> ItemType;

    /*
    // HACK : Work around Rust's lack of trait reflection/conversion at runtime
    fn  get_item_action_points  (&self) -> u32;
    fn  get_item_action_text    (&self) -> String;
    fn  as_weapon               (&self) -> Option<Rc<&dyn Weapon>>;
    fn  as_shield               (&self) -> Option<Rc<&dyn Shield>>;
    */
}

#[macro_export]
macro_rules! impl_Item { 
    ($T:ident) => {
        impl Item for $T {
            fn get_item_weight(&self) -> u32 {
                 return self.item_data.weight; 
            }

            fn get_item_property(&self) -> ItemProperty {
                return self.item_data.property; 
            }

            fn get_item_type(&self) -> ItemType {
                return self.item_data.item_type; 
            }

            /*
            fn get_item_action_points  (&self) -> u32 {
                return u32::max_value();
            }

            fn get_item_action_text    (&self) -> String {
                return "This should be overridden.".to_string();
            }

            fn as_weapon (&self) -> Option<Rc<&dyn crate::game::weapon::Weapon>> {
                return Option::None;
            }

            fn as_shield (&self) -> Option<Rc<&dyn crate::game::shield::Shield>> {
                return Option::None;
            }
            */
        }
    }
}