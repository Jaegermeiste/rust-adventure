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
use crate::game::items::item::*;

pub static	GOAL_NAME		: &str = "Black Forest Cake";
static	GOAL_FLAVOR_TEXT	: &str = " The cake is not a lie. This fabled MacGuffin has somehow been freshly baked and frosted by mysterious forces unknown.";
static	GOAL_WEIGHT			: u32 = 0;

pub struct Goal {
    // GameObject
    game_object_data    : GameObjectData,

    // Item
    item_data           : ItemData,
}

crate::impl_GameObject!(Goal);

crate::impl_Item!(Goal);

impl  Default for Goal {
    fn default() -> Goal {
        let result = Goal {
            game_object_data    : GameObjectData    { name : String::from(GOAL_NAME), flavor_text : String::from(GOAL_FLAVOR_TEXT) },
            item_data           : ItemData          { weight : GOAL_WEIGHT, property : ItemProperty::Droppable, item_type : ItemType::GameWin },
        };
        return result;
    }
}