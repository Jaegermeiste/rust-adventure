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

static	SCUTUM_NAME					: &str = "Scutum";
static	SCUTUM_FLAVOR_TEXT			: &str = " Pronounced scoo-tem, this shield is extremely sturdy.";
static	SCUTUM_WEIGHT				: u32 = 22;
static	SCUTUM_DEFENSE_POINTS		: u32 = 50;
static  SCUTUM_DEFENSE_MODE_TEXT	: &str = "deflects";

pub struct Scutum {
    // GameObject
    game_object_data    : GameObjectData,

    // Item
    item_data           : ItemData,

    // Shield
    shield_data         : ShieldData,
}

crate::impl_Shield!(Scutum);

impl  Default for Scutum {
    fn default() -> Scutum {
        let result = Scutum {
            game_object_data    : GameObjectData    { name : String::from(SCUTUM_NAME), flavor_text : String::from(SCUTUM_FLAVOR_TEXT) },
            item_data           : ItemData          { weight : SCUTUM_WEIGHT, property : ItemProperty::Droppable, item_type : ItemType::Shield },
            shield_data         : ShieldData        { defense_points : SCUTUM_DEFENSE_POINTS, defense_mode_text : String::from(SCUTUM_DEFENSE_MODE_TEXT) },
        };
        return result;
    }
}