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
use crate::game::weapon::*;

static	RATTOOTH_NAME				: &str = "Rat Tooth";
static	RATTOOTH_FLAVOR_TEXT		: &str = " The bacterial plaque is more likely to do damage than the bite.";
static	RATTOOTH_WEIGHT				: u32 = 1;
static  RATTOOTH_ATTACK_POINTS		: u32 = 5;
static  RATTOOTH_ATTACK_MODE_TEXT	: &str = "bites";

pub struct RatTooth {
    // GameObject
    game_object_data    : GameObjectData,

    // Item
    item_data           : ItemData,

    // Weapon
    weapon_data         : WeaponData,
}

crate::impl_Weapon!(RatTooth);

impl  Default for RatTooth {
    fn default() -> RatTooth {
        let result = RatTooth {
            game_object_data    : GameObjectData    { name : String::from(RATTOOTH_NAME), flavor_text : String::from(RATTOOTH_FLAVOR_TEXT) },
            item_data           : ItemData          { weight : RATTOOTH_WEIGHT, property : ItemProperty::Droppable, item_type : ItemType::Weapon },
            weapon_data         : WeaponData        { attack_points : RATTOOTH_ATTACK_POINTS, attack_mode_text : String::from(RATTOOTH_ATTACK_MODE_TEXT) },
        };
        return result;
    }
}