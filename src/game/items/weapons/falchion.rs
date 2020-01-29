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

static	FALCHION_NAME				: &str = "Falchion";
static	FALCHION_FLAVOR_TEXT		: &str = " A glorified machete.";
static	FALCHION_WEIGHT				: u32 = 3;
static	FALCHION_ATTACK_POINTS		: u32 = 25;
static  FALCHION_ATTACK_MODE_TEXT	: &str = "slices";

pub struct Falchion {
    // GameObject
    game_object_data    : GameObjectData,

    // Item
    item_data           : ItemData,

    // Weapon
    weapon_data         : WeaponData,
}

crate::impl_Weapon!(Falchion);

impl  Default for Falchion {
    fn default() -> Falchion {
        let result = Falchion {
            game_object_data    : GameObjectData    { name : String::from(FALCHION_NAME), flavor_text : String::from(FALCHION_FLAVOR_TEXT) },
            item_data           : ItemData          { weight : FALCHION_WEIGHT, property : ItemProperty::Droppable, item_type : ItemType::Weapon },
            weapon_data         : WeaponData        { attack_points : FALCHION_ATTACK_POINTS, attack_mode_text : String::from(FALCHION_ATTACK_MODE_TEXT) },
        };
        return result;
    }
}