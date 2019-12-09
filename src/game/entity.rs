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
use traitcast::{TraitcastFrom};
use crate::game::gameobject::*;
use crate::game::locatable::*;

static ATTACK_MIN_SCALE     : f32 = 0.1;
static ATTACK_MAX_SCALE     : f32 =	1.3;

static DEFENSE_MIN_SCALE    : f32 =	0.1;
static DEFENSE_MAX_SCALE    : f32 =	1.1;

pub struct EntityData {
    pub pack        : Backpack,

    pub health      : i32,
    pub max_health  : i32,

    pub x_coord     : u32,
    pub y_coord     : u32,

    pub alive       : bool,

    pub xp          : u32,
}

impl Default for EntityData {
    fn default() -> EntityData {
        let data = EntityData {
            pack        : Backpack,

            health      : 1,
            max_health  : 1,
        
            x_coord     : 0,
            y_coord     : 0,
        
            alive       : true,
        
            xp          : 0,
        };
        return data;
    }
}

pub trait Entity: GameObject + Locatable + TraitcastFrom {
    fn  health                  (&self) -> i32;
    fn  max_health              (&self) -> i32;
    fn  add_health              (&self, additional_health : i32);
    fn  alive                   (&self) -> bool;

    fn  attack                  (&self) -> u32;
    fn  defend                  (&self, attack_strength : u32) -> u32;

    fn  list_pack_items         (&self);
    fn  get_pack_item_for_index (&self, index : usize) -> Rc<dyn Item>;
    fn  has_pack_item_of_type   (&self, item_type : ItemType) -> bool;

    fn  xp                      (&self) -> u32;

    fn  pack                    (&self) -> Option<Rc<Backpack>>;
}

#[macro_export]
macro_rules! impl_Entity { 
    ($T:ident) => {
        impl Entity for $T {
            fn health (&self) -> i32 {
                 return self.entity_data.health; 
            }

            fn max_health (&self) -> i32 {
                return self.entity_data.max_health;
            }

            fn add_health (&self, additional_health : i32) {
                let result = self.entity_data.health.checked_add(additional_health);

                if result.is_none() || 
                    (self.entity_data.health > self.entity_data.max_health) {
                        self.entity_data.health = self.entity_data.max_health;
                    }
            }

            fn alive (&self) -> bool {
                return self.entity_data.alive;
            }

            fn attack (&self) -> u32 {
                return 0;           // FIXME
            }

            fn defend (&self, attack_strength : u32) -> u32 {
                return 0;           // FIXME
            }

            fn list_pack_items (&self) {
                self.entity_data.pack.print_items();
            }

            fn get_pack_item_for_index (&self, index : usize) -> Rc<dyn Item> {
                return self.entity_data.pack.drop_item(index);
            }

            fn has_pack_item_of_type (&self, item_type : ItemType) -> bool {
                return self.entity_data.pack.item_type_exists(item_type);
            }

            fn xp (&self) -> u32 {
                return self.entity_data.xp;
            }

            fn pack (&self) -> Option<Rc<Backpack>> {
                return Option::Some(Rc::new(self.entity_data.pack));;
            }
        }
    }
}