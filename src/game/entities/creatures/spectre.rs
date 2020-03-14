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
use rand::distributions::{Distribution, Bernoulli};
use crate::game::gameobject::*;
use crate::game::locatable::*;
use crate::game::entities::entity::*;
use crate::game::entities::creatures::creature::*;
use crate::game::items::weapons::creaturespecific::spectrelifesap::*;
use crate::game::items::other::healthpotion::*;

pub static	SPECTRE_NAME		: &str = "Spectre";
static	SPECTRE_FLAVOR_TEXT	: &str = " WoooOOOOOOooooo. SCARY.";
static	SPECTRE_MAX_HEALTH		: i32 = 30;

pub struct Spectre {
    game_object_data    : GameObjectData,
    location_data       : LocationData,
    entity_data         : EntityData
}

crate::impl_Creature!(Spectre);

impl  Default for Spectre {
    fn default() -> Spectre {
        let mut result = Spectre {
            game_object_data    : GameObjectData    { name : String::from(SPECTRE_NAME), flavor_text : String::from(SPECTRE_FLAVOR_TEXT) },
            location_data       : LocationData      { x_coord : 0, y_coord : 0 },
            entity_data         : EntityData        { health : SPECTRE_MAX_HEALTH, max_health : SPECTRE_MAX_HEALTH, ..Default::default() },
        };

        // Give the Spectre its special weapon
        result.entity_data.pack.add_item(Rc::new(SpectreLifeSap::default()));
        result.entity_data.pack.set_weapon(0, false);

        // Add health to inventory, maybe
        let mut rng = rand::thread_rng();
        let health_toss = Bernoulli::new(CREATURE_HEALTH_DROP_PCT).unwrap();
        if health_toss.sample(&mut rng) {
            result.entity_data.pack.add_item(Rc::new(HealthPotion::default()));
        }

        return result;
    }
}