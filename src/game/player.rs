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
use crate::game::entity::*;
use crate::game::dirk::*;
use crate::game::buckler::*;
use crate::game::healthpotion::*;

static PLAYER_NAME          : &str = "Chain";

static PLAYER_FLAVOR_TEXT	: &str = " Definitely not Zelda.";

static PLAYER_XP_LEVEL_1	: u32 = 100;	// Rats
static PLAYER_XP_LEVEL_2	: u32 = 500;	// Spectre
static PLAYER_XP_LEVEL_3	: u32 = 1000;	// Skeleton
static PLAYER_XP_LEVEL_4	: u32 = 3000;	// Demogorgon

static PLAYER_MAX_HEALTH	: i32 = 100;

pub struct Player {
    game_object_data    : GameObjectData,
    location_data       : LocationData,
    entity_data         : EntityData
}

impl Player {
    pub fn new() -> Player {
        let mut player = Player { 
            game_object_data    : GameObjectData    { name : String::from(PLAYER_NAME), flavor_text : String::from(PLAYER_FLAVOR_TEXT) },
            location_data       : LocationData      { x_coord : 0, y_coord : 0 },
            entity_data         : EntityData        { health : PLAYER_MAX_HEALTH, max_health : PLAYER_MAX_HEALTH, ..Default::default() },
            };

        // Add Dirk
        let pointer_weapon = Rc::new(Dirk::default());
        player.entity_data.pack.add_item(pointer_weapon);
        player.entity_data.pack.set_weapon(0, false);

        // Add buckler
        let pointer_shield = Rc::new(Buckler::default());
        player.entity_data.pack.add_item(pointer_shield);
        player.entity_data.pack.set_shield(1, false);

        // Add Health Potion
        let pointer_health = Rc::new(HealthPotion::default());
        player.entity_data.pack.add_item(pointer_health);

        return player;
    }
}

crate::impl_Entity!(Player);