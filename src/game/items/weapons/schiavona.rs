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

static	SCHIAVONA_NAME				: &str = "Schiavona";
static	SCHIAVONA_FLAVOR_TEXT		: &str = " An elegant weapon from another age.";
static	SCHIAVONA_WEIGHT			: u32 = 3;
static	SCHIAVONA_ATTACK_POINTS		: u32 = 50;
static  SCHIAVONA_ATTACK_MODE_TEXT	: &str = "slices";

pub struct Schiavona {
    // GameObject
    game_object_data    : GameObjectData,

    // Item
    item_data           : ItemData,

    // Weapon
    weapon_data         : WeaponData,
}

crate::impl_Weapon!(Schiavona);

impl  Default for Schiavona {
    fn default() -> Schiavona {
        let result = Schiavona {
            game_object_data    : GameObjectData    { name : String::from(SCHIAVONA_NAME), flavor_text : String::from(SCHIAVONA_FLAVOR_TEXT) },
            item_data           : ItemData          { weight : SCHIAVONA_WEIGHT, property : ItemProperty::Droppable, item_type : ItemType::Weapon },
            weapon_data         : WeaponData        { attack_points : SCHIAVONA_ATTACK_POINTS, attack_mode_text : String::from(SCHIAVONA_ATTACK_MODE_TEXT) },
        };
        return result;
    }
}