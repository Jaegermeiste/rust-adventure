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
use traitcast::{TraitcastFrom};
extern crate rand;
use crate::game::gameobject::*;
use crate::game::locatable::*;
use crate::game::item::*;
use crate::game::backpack::*;

pub static ATTACK_MIN_SCALE     : f32 = 0.1;
pub static ATTACK_MAX_SCALE     : f32 =	1.3;

pub static DEFENSE_MIN_SCALE    : f32 =	0.1;
pub static DEFENSE_MAX_SCALE    : f32 =	1.1;

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
            pack        : Backpack::new(),

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
    fn  add_health              (&mut self, additional_health : i32);
    fn  alive                   (&self) -> bool;

    fn  attack                  (&mut self) -> u32;
    fn  defend                  (&mut self, attack_strength : u32) -> u32;

    fn  list_pack_items         (&self);
    fn  get_pack_item_for_index (&mut self, index : usize) -> Rc<dyn Item>;
    fn  has_pack_item_of_type   (&self, item_type : ItemType) -> bool;

    fn  xp                      (&self) -> u32;

    fn  pack                    (&self) -> Option<Rc<&Backpack>>;
}

#[macro_export]
macro_rules! impl_Entity { 
    ($T:ident) => {
        extern crate rand;
        use rand::Rng;

        crate::impl_GameObject!($T);

        crate::impl_Locatable!($T);

        impl Entity for $T {
            fn health (&self) -> i32 {
                 return self.entity_data.health; 
            }

            fn max_health (&self) -> i32 {
                return self.entity_data.max_health;
            }

            fn add_health (&mut self, additional_health : i32) {
                let result = self.entity_data.health.checked_add(additional_health);

                if result.is_none() || 
                    (self.entity_data.health > self.entity_data.max_health) {
                        self.entity_data.health = self.entity_data.max_health;
                    }
            }

            fn alive (&self) -> bool {
                return self.entity_data.alive;
            }

            fn attack (&mut self) -> u32 {
                let mut attack_roll :u32 = 0;
                let weapon_opt = self.entity_data.pack.current_weapon();
                let mut random_device = rand::thread_rng();
                let	scalar = rand::distributions::Uniform::new(crate::game::entity::ATTACK_MIN_SCALE, crate::game::entity::ATTACK_MAX_SCALE);
            
                if weapon_opt.is_some() {
                    let weapon = weapon_opt.unwrap();

                    print!("   {}, using {}, rolls ", self.game_object_data.name, weapon.name());
            
                    // Scale the weapon strength by the random scalar
                    attack_roll = ((weapon.attack_points() as f64) * (random_device.sample(scalar) as f64)).round() as u32;
            
                    println!("{} points for attack!", attack_roll);
            
                    // Declare critical hit, if it happened
                    if attack_roll > weapon.attack_points() {
                        println!("          Critical Hit!");
                    }
            
                    // Add to experience
                    self.entity_data.xp += attack_roll;
                }
                else {  // weapon_opt is None
                    println!("   {} does not have a weapon equipped, and cannot attack!", self.game_object_data.name);
                }
            
                return attack_roll;
            }

            fn defend (&mut self, attack_strength : u32) -> u32 {
                let mut defense_roll = 0;
                let mut damage_taken : i32;
                let shield_opt = self.entity_data.pack.current_shield();
                let mut random_device = rand::thread_rng();
                let	scalar = rand::distributions::Uniform::new(crate::game::entity::DEFENSE_MIN_SCALE, crate::game::entity::DEFENSE_MAX_SCALE);
            
                if shield_opt.is_some() {
                    let shield = shield_opt.unwrap();

                    print!("   {}, using {}, rolls ", self.game_object_data.name, shield.name());

                    // roll the specified dice
                    defense_roll = ((shield.defense_points() as f64) * (random_device.sample(scalar) as f64)).round() as u32;
                    
                    if defense_roll == 1 {
                        // Singular
                        println!("{}  for defense, blocking {} point of damage!", defense_roll, defense_roll);
                    }
                    else {
                        // Plural
                        println!("{}  for defense, blocking {} points of damage!", defense_roll, defense_roll);
                    }
            
                    // Add to experience
                    self.entity_data.xp += defense_roll;
                }
                else {  // shield_opt is None
                    println!("   {} does not have a shield equipped, and cannot defend!", self.game_object_data.name);
                }
            
                // subtract the Defense roll from the Attack roll
                damage_taken = (attack_strength - defense_roll) as i32;
            
                // Clamp - feature unstable as of 20191208, so fake it
                //damage_taken.clamp(0, self.entity_data.health);
                if damage_taken < 0 {
                    damage_taken = 0;
                }
                else if damage_taken > self.entity_data.health {
                    damage_taken = self.entity_data.health;
                }
            
                println!("   {} took {} points of damage.", self.game_object_data.name, damage_taken);
            
                // The result is then subtracted from the Strength Points.
                self.entity_data.health -= damage_taken;
            
                // Clamp - feature unstable as of 20191208, so fake it
                //self.entity_data.health.clamp(0, self.entity_data.max_health);
                if self.entity_data.health < 0 {
                    self.entity_data.health = 0;
                }
                else if self.entity_data.health > self.entity_data.max_health {
                    self.entity_data.health = self.entity_data.max_health;
                }

                println!("   {} has {} health.", self.game_object_data.name, self.entity_data.health);
            
                if self.entity_data.health <= 0 {
                    println!("   {} has died.", self.game_object_data.name);
                    self.entity_data.alive = false;
                }
            
                return self.entity_data.health as u32;
            }

            fn list_pack_items (&self) {
                self.entity_data.pack.print_items();
            }

            fn get_pack_item_for_index (&mut self, index : usize) -> Rc<dyn crate::game::item::Item> {
                return self.entity_data.pack.drop_item(index).unwrap();
            }

            fn has_pack_item_of_type (&self, item_type : crate::game::item::ItemType) -> bool {
                return self.entity_data.pack.item_type_exists(item_type);
            }

            fn xp (&self) -> u32 {
                return self.entity_data.xp;
            }

            fn pack (&self) -> Option<Rc<&crate::game::backpack::Backpack>> {
                return Option::Some(Rc::new(&self.entity_data.pack));
            }
        }
    }
}