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

pub trait Item: GameObject {
    fn  get_item_weight     (&self) -> u32;
    fn  get_item_property   (&self) -> ItemProperty;
    fn  get_item_type       (&self) -> ItemType;
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
        }
    }
}