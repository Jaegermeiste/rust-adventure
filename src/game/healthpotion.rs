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
use traitcast::{traitcast};
use crate::game::gameobject::*;
use crate::game::item::*;

static	HEALTHPOTION_NAME			: &str = "Health Potion";
static	HEALTHPOTION_FLAVOR_TEXT	: &str = " This concoction of creatine and taurine makes your heart race, but doesn't give you wings.";
static	HEALTHPOTION_WEIGHT			: u32 = 1;
static  HEALTHPOTION_VALUE			: u32 = 10;

pub struct HealthPotion {
    // GameObject
    game_object_data    : GameObjectData,

    // Item
    item_data           : ItemData,
}

crate::impl_GameObject!(HealthPotion);

crate::impl_Item!(HealthPotion);

impl  Default for HealthPotion {
    fn default() -> HealthPotion {
        let result = HealthPotion {
            game_object_data    : GameObjectData    { name : String::from(HEALTHPOTION_NAME), flavor_text : String::from(HEALTHPOTION_FLAVOR_TEXT) },
            item_data           : ItemData          { weight : HEALTHPOTION_WEIGHT, property : ItemProperty::Droppable, item_type : ItemType::Health },
        };
        return result;
    }
}