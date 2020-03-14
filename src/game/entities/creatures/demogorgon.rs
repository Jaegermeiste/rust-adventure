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
use std::rc::Rc;
use crate::game::gameobject::*;
use crate::game::locatable::*;
use crate::game::entities::entity::*;
use crate::game::entities::creatures::creature::*;
use crate::game::items::weapons::creaturespecific::demogorgonupsidedown::*;
use crate::game::items::weapons::swordofathousandtruths::*;
use crate::game::items::other::healthpotion::*;
use crate::game::items::other::throneroomkey::*;

pub static	DEMOGORGON_NAME		: &str = "Demogorgon";
static	DEMOGORGON_FLAVOR_TEXT	: &str = " A creature of unknown origin that definitely knows nothing about the Department of Energy or the number Eleven.";
static	DEMOGORGON_MAX_HEALTH		: i32 = 100;

pub struct Demogorgon {
    game_object_data    : GameObjectData,
    location_data       : LocationData,
    entity_data         : EntityData
}

crate::impl_Creature!(Demogorgon);

impl  Default for Demogorgon {
    fn default() -> Demogorgon {
        let mut result = Demogorgon {
            game_object_data    : GameObjectData    { name : String::from(DEMOGORGON_NAME), flavor_text : String::from(DEMOGORGON_FLAVOR_TEXT) },
            location_data       : LocationData      { x_coord : 0, y_coord : 0 },
            entity_data         : EntityData        { health : DEMOGORGON_MAX_HEALTH, max_health : DEMOGORGON_MAX_HEALTH, ..Default::default() },
        };

        // Wrath of Barb
        result.entity_data.pack.add_item(Rc::new(DemogorgonUpsideDown::default()));
        result.entity_data.pack.set_weapon(0, false);

        // Add throne room key
        result.entity_data.pack.add_item(Rc::new(ThroneRoomKey::default()));

        // Add a bonus, just because
        result.entity_data.pack.add_item(Rc::new(SwordOfAThousandTruths::default()));

        // Add health to inventory
        result.entity_data.pack.add_item(Rc::new(HealthPotion::default()));

        return result;
    }
}