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

static	SWORDOFATHOUSANDTRUTHS_NAME					: &str = "Sword of a Thousand Truths";
static	SWORDOFATHOUSANDTRUTHS_FLAVOR_TEXT			: &str = " As foretold by Salzman in Accounting.";
static	SWORDOFATHOUSANDTRUTHS_WEIGHT				: u32 = 5;
static	SWORDOFATHOUSANDTRUTHS_ATTACK_POINTS		: u32 = 100;
static  SWORDOFATHOUSANDTRUTHS_ATTACK_MODE_TEXT		: &str = "imparts divine truth";

pub struct SwordOfAThousandTruths {
    // GameObject
    game_object_data    : GameObjectData,

    // Item
    item_data           : ItemData,

    // Weapon
    weapon_data         : WeaponData,
}

crate::impl_Weapon!(SwordOfAThousandTruths);

impl  Default for SwordOfAThousandTruths {
    fn default() -> SwordOfAThousandTruths {
        let result = SwordOfAThousandTruths {
            game_object_data    : GameObjectData    { name : String::from(SWORDOFATHOUSANDTRUTHS_NAME), flavor_text : String::from(SWORDOFATHOUSANDTRUTHS_FLAVOR_TEXT) },
            item_data           : ItemData          { weight : SWORDOFATHOUSANDTRUTHS_WEIGHT, property : ItemProperty::Droppable, item_type : ItemType::Weapon },
            weapon_data         : WeaponData        { attack_points : SWORDOFATHOUSANDTRUTHS_ATTACK_POINTS, attack_mode_text : String::from(SWORDOFATHOUSANDTRUTHS_ATTACK_MODE_TEXT) },
        };
        return result;
    }
}