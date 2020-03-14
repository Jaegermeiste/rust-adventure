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
use crate::game::items::weapons::falchion::*;
use crate::game::items::weapons::dirk::*;
use crate::game::items::other::healthpotion::*;

pub static	SKELETON_NAME		: &str = "Skeleton";
static	SKELETON_FLAVOR_TEXT	: &str = " A bony opponent equipped with a ";
static	SKELETON_COIN_FLIP_PCT	: f64 = 0.5;
static	SKELETON_MAX_HEALTH		: i32 = 50;

pub struct Skeleton {
    game_object_data    : GameObjectData,
    location_data       : LocationData,
    entity_data         : EntityData
}

crate::impl_Creature!(Skeleton);

impl  Default for Skeleton {
    fn default() -> Skeleton {
        let mut rng = rand::thread_rng();

        // Pick weapon
        let coin = Bernoulli::new(SKELETON_COIN_FLIP_PCT).unwrap();
        let coin_flip : bool = coin.sample(&mut rng);
        let flavor_str;
        if coin_flip {
            flavor_str = format!("{}{}", SKELETON_FLAVOR_TEXT, FALCHION_NAME);
        }
        else {
            flavor_str = format!("{}{}", SKELETON_FLAVOR_TEXT, DIRK_NAME);
        }

        let mut result = Skeleton {
            game_object_data    : GameObjectData    { name : String::from(SKELETON_NAME), flavor_text : flavor_str },
            location_data       : LocationData      { x_coord : 0, y_coord : 0 },
            entity_data         : EntityData        { health : SKELETON_MAX_HEALTH, max_health : SKELETON_MAX_HEALTH, ..Default::default() },
        };

        let mut rng = rand::thread_rng();

        // Actually add weapon to pack
        if coin_flip {
            result.entity_data.pack.add_item(Rc::new(Falchion::default()));
        }
        else {
            result.entity_data.pack.add_item(Rc::new(Dirk::default()));
        }
        result.entity_data.pack.set_weapon(0, false);

        // Add health to inventory, maybe
        let health_toss = Bernoulli::new(CREATURE_HEALTH_DROP_PCT).unwrap();
        if health_toss.sample(&mut rng) {
            result.entity_data.pack.add_item(Rc::new(HealthPotion::default()));
        }

        return result;
    }
}