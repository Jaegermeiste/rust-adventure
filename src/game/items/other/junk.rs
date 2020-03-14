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

pub static	JUNK_NAME			: &str = "Junk";
static	JUNK_FLAVOR_TEXT	: &str = " An assortment of random objects. Might be worth something to a peddler, if there were any around this cursed place.";
static	JUNK_WEIGHT			: u32 = 3;

pub struct Junk {
    // GameObject
    game_object_data    : GameObjectData,

    // Item
    item_data           : ItemData,
}

crate::impl_GameObject!(Junk);

crate::impl_Item!(Junk);

impl  Default for Junk {
    fn default() -> Junk {
        let result = Junk {
            game_object_data    : GameObjectData    { name : String::from(JUNK_NAME), flavor_text : String::from(JUNK_FLAVOR_TEXT) },
            item_data           : ItemData          { weight : JUNK_WEIGHT, property : ItemProperty::Droppable, ..Default::default() },
        };
        return result;
    }
}