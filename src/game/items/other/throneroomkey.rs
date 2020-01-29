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

static	THRONEROOMKEY_NAME			: &str = "Throne Room Key";
static	THRONEROOMKEY_FLAVOR_TEXT	: &str = " This ornate skeleton-style key gives you access to your goal.";
static	THRONEROOMKEY_WEIGHT		: u32 = 0;

pub struct ThroneRoomKey {
    // GameObject
    game_object_data    : GameObjectData,

    // Item
    item_data           : ItemData,
}

crate::impl_GameObject!(ThroneRoomKey);

crate::impl_Item!(ThroneRoomKey);

impl  Default for ThroneRoomKey {
    fn default() -> ThroneRoomKey {
        let result = ThroneRoomKey {
            game_object_data    : GameObjectData    { name : String::from(THRONEROOMKEY_NAME), flavor_text : String::from(THRONEROOMKEY_FLAVOR_TEXT) },
            item_data           : ItemData          { weight : THRONEROOMKEY_WEIGHT, property : ItemProperty::Droppable, item_type : ItemType::ThroneRoomKey },
        };
        return result;
    }
}