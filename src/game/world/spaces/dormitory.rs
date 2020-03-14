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
use std::cell::RefCell;
use rand::distributions::{Distribution, Uniform};
use traitcast::{traitcast};
use crate::game::gameobject::*;
use crate::game::locatable::*;
use crate::game::world::spaces::space::*;
use crate::game::items::backpack::*;
use crate::game::items::weapons::dirk::*;
use crate::game::items::weapons::falchion::*;
use crate::game::items::weapons::schiavona::*;
use crate::game::items::weapons::swordofathousandtruths::*;
use crate::game::items::shields::buckler::*;
use crate::game::items::shields::heater::*;
use crate::game::items::shields::scutum::*;
use crate::game::items::other::healthpotion::*;

static  DORMITORY_NAME					: &str = "Guard Barracks";
static  DORMITORY_DOORS				    : u32 = 2;
static  DORMITORY_FLAVOR_TEXT			: &str = " A utilitarian space. The frameworks of the soldier's bunks remain, but everything else has been reduced to dust.";
static  DORMITORY_SPECIAL_ACTION_TEXT 	: &str = "Open Footlocker.";

static	DORMITORY_DIRK_DROP_PCT		: u32 = 30;
static	DORMITORY_FALCHION_DROP_PCT	: u32 = DORMITORY_DIRK_DROP_PCT + 20;
static	DORMITORY_SCHIAVONA_DROP_PCT	: u32 = DORMITORY_FALCHION_DROP_PCT + 20;
static	DORMITORY_SWORD1000_DROP_PCT	: u32 = DORMITORY_SCHIAVONA_DROP_PCT + 20;

static DORMITORY_BUCKLER_DROP_PCT		: u32 = 30;
static DORMITORY_HEATER_DROP_PCT		: u32 = DORMITORY_BUCKLER_DROP_PCT + 20;
static DORMITORY_SCUTUM_DROP_PCT		: u32 = DORMITORY_HEATER_DROP_PCT + 20;

pub struct Dormitory {
    // GameObject
    game_object_data    : GameObjectData,

    //Location
    location_data       : LocationData,

    // Space
    space_data          : SpaceData,
}

crate::impl_Space!(Dormitory);

impl  Default for Dormitory {
    fn default() -> Dormitory {
        let result = Dormitory {
            game_object_data    : GameObjectData    { name : String::from(DORMITORY_NAME), flavor_text : String::from(DORMITORY_FLAVOR_TEXT) },
            location_data       : LocationData      { ..Default::default()  },
            space_data          : SpaceData         { 
                                                        special_action_text : String::from(DORMITORY_SPECIAL_ACTION_TEXT),
                                                        movement_enabled    : true,
                                                        space_type          : SpaceType::Dormitory,
                                                        ..Default::default()
                                                    },
        };
        return result;
    }
}

impl Dormitory {
    fn perform_special_action(&mut self) {
        let mut rng = rand::thread_rng();
        let item_type_roll = Uniform::from(0..=100);

        // Create a weapon
        let mut roll_result : u32 = item_type_roll.sample(&mut rng);

        if roll_result >= DORMITORY_SWORD1000_DROP_PCT {
            self.space_data.pack.borrow_mut().add_item(Rc::new(SwordOfAThousandTruths::default()));
        }
        else if roll_result >= DORMITORY_SCHIAVONA_DROP_PCT {
            self.space_data.pack.borrow_mut().add_item(Rc::new(Schiavona::default()));
        }
        else if roll_result >= DORMITORY_FALCHION_DROP_PCT {
            self.space_data.pack.borrow_mut().add_item(Rc::new(Falchion::default()));
        }
        else { //if roll_result >= DORMITORY_DIRK_DROP_PCT {
            self.space_data.pack.borrow_mut().add_item(Rc::new(Dirk::default()));
        }

        // Create a shield
        roll_result = item_type_roll.sample(&mut rng);

        if roll_result >= DORMITORY_SCUTUM_DROP_PCT {
            self.space_data.pack.borrow_mut().add_item(Rc::new(Scutum::default()));
        }
        else if roll_result >= DORMITORY_HEATER_DROP_PCT {
            self.space_data.pack.borrow_mut().add_item(Rc::new(Heater::default()));
        }
        else { //if roll_result >= DORMITORY_BUCKLER_DROP_PCT {
            self.space_data.pack.borrow_mut().add_item(Rc::new(Buckler::default()));
        }

        // Also add a health potion
        self.space_data.pack.borrow_mut().add_item(Rc::new(HealthPotion::default()));

        println!(" The footloacker is now open.");
        
        self.space_data.special_action_performed = true;
    }
}