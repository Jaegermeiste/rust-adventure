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

static	HEATER_NAME					: &str = "Heater";
static	HEATER_FLAVOR_TEXT			: &str = " Despite the name, this does not keep you warm.";
static	HEATER_WEIGHT				: u32 = 15;
static	HEATER_DEFENSE_POINTS		: u32 = 30;
static  HEATER_DEFENSE_MODE_TEXT	: &str = "deflects";

pub struct Heater {
    // GameObject
    game_object_data    : GameObjectData,

    // Item
    item_data           : ItemData,

    // Shield
    shield_data         : ShieldData,
}

crate::impl_Shield!(Heater);

impl  Default for Heater {
    fn default() -> Heater {
        let result = Heater {
            game_object_data    : GameObjectData    { name : String::from(HEATER_NAME), flavor_text : String::from(HEATER_FLAVOR_TEXT) },
            item_data           : ItemData          { weight : HEATER_WEIGHT, property : ItemProperty::Droppable, item_type : ItemType::Shield },
            shield_data         : ShieldData        { defense_points : HEATER_DEFENSE_POINTS, defense_mode_text : String::from(HEATER_DEFENSE_MODE_TEXT) },
        };
        return result;
    }
}