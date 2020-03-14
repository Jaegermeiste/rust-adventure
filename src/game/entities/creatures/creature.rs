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
use crate::game::entities::entity::*;

pub static	CREATURE_HEALTH_DROP_PCT       : f64   = 0.6;

/*pub struct Creature {
    game_object_data    : GameObjectData,
    location_data       : LocationData,
    entity_data         : EntityData
}*/

pub trait Creature: Entity {
}

#[macro_export]
macro_rules! impl_Creature { 
    ($T:ident) => {
        crate::impl_Entity!($T);

        impl Creature for $T {
        }
    }
}