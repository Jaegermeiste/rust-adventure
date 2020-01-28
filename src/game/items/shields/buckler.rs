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
use crate::game::items::item::*;
use crate::game::items::shields::shield::*;

static	BUCKLER_NAME				: &str = "Buckler";
static	BUCKLER_FLAVOR_TEXT			: &str = " A tiny shield the size of a dinner plate.";
static	BUCKLER_WEIGHT				: u32 = 5;
static	BUCKLER_DEFENSE_POINTS		: u32 = 10;
static  BUCKLER_DEFENSE_MODE_TEXT	: &str = "deflects";

pub struct Buckler {
    // GameObject
    game_object_data    : GameObjectData,

    // Item
    item_data           : ItemData,

    // Shield
    shield_data         : ShieldData,
}

crate::impl_Shield!(Buckler);

impl  Default for Buckler {
    fn default() -> Buckler {
        let result = Buckler {
            game_object_data    : GameObjectData    { name : String::from(BUCKLER_NAME), flavor_text : String::from(BUCKLER_FLAVOR_TEXT) },
            item_data           : ItemData          { weight : BUCKLER_WEIGHT, property : ItemProperty::Droppable, item_type : ItemType::Shield },
            shield_data         : ShieldData        { defense_points : BUCKLER_DEFENSE_POINTS, defense_mode_text : String::from(BUCKLER_DEFENSE_MODE_TEXT) },
        };
        return result;
    }
}