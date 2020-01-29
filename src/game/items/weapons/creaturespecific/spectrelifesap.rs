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
use crate::game::items::weapons::weapon::*;

static	SPECTRELIFESAP_NAME				: &str = "Life Sap";
static	SPECTRELIFESAP_FLAVOR_TEXT		: &str = " Imparts a cold sense of impending doom.";
static	SPECTRELIFESAP_WEIGHT			: u32 = 0;
static  SPECTRELIFESAP_ATTACK_POINTS	: u32 = 15;
static  SPECTRELIFESAP_ATTACK_MODE_TEXT	: &str = "saps";

pub struct SpectreLifeSap {
    // GameObject
    game_object_data    : GameObjectData,

    // Item
    item_data           : ItemData,

    // Weapon
    weapon_data         : WeaponData,
}

crate::impl_Weapon!(SpectreLifeSap);

impl  Default for SpectreLifeSap {
    fn default() -> SpectreLifeSap {
        let result = SpectreLifeSap {
            game_object_data    : GameObjectData    { name : String::from(SPECTRELIFESAP_NAME), flavor_text : String::from(SPECTRELIFESAP_FLAVOR_TEXT) },
            item_data           : ItemData          { weight : SPECTRELIFESAP_WEIGHT, property : ItemProperty::None, item_type : ItemType::Weapon },
            weapon_data         : WeaponData        { attack_points : SPECTRELIFESAP_ATTACK_POINTS, attack_mode_text : String::from(SPECTRELIFESAP_ATTACK_MODE_TEXT) },
        };
        return result;
    }
}