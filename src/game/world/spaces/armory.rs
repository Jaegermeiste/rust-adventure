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
use crate::game::items::item::*;
use crate::game::items::weapons::weapon::*;
use crate::game::items::weapons::dirk::*;
use crate::game::items::weapons::falchion::*;
use crate::game::items::weapons::schiavona::*;
use crate::game::items::weapons::swordofathousandtruths::*;
use crate::game::items::shields::shield::*;
use crate::game::items::shields::buckler::*;
use crate::game::items::shields::heater::*;
use crate::game::items::shields::scutum::*;

static  ARMORY_NAME					: &str = "Armory";
static  ARMORY_DOORS				: u32 = 2;
static  ARMORY_FLAVOR_TEXT			: &str = " Under the odor of must and mold, you detect the slightest whiff of oil.";
static  ARMORY_SPECIAL_ACTION_TEXT 	: &str = "Break chains on Weapons Rack";

static	ARMORY_DIRK_DROP_PCT		: u32 = 30;
static	ARMORY_FALCHION_DROP_PCT	: u32 = 50;
static	ARMORY_SCHIAVONA_DROP_PCT	: u32 = 70;
static	ARMORY_SWORD1000_DROP_PCT	: u32 = 90;

static ARMORY_BUCKLER_DROP_PCT		: u32 = 30;
static ARMORY_HEATER_DROP_PCT		: u32 = 50;
static ARMORY_SCUTUM_DROP_PCT		: u32 = 70;

pub struct Armory {
    // GameObject
    game_object_data    : GameObjectData,

    //Location
    location_data       : LocationData,

    // Space
    space_data          : SpaceData,
}

crate::impl_Space!(Armory);

impl  Default for Armory {
    fn default() -> Armory {
        let result = Armory {
            game_object_data    : GameObjectData    { name : String::from(ARMORY_NAME), flavor_text : String::from(ARMORY_FLAVOR_TEXT) },
            location_data       : LocationData      { ..Default::default()  },
            space_data          : SpaceData         { 
                                                        special_action_text : String::from(ARMORY_SPECIAL_ACTION_TEXT),
                                                        movement_enabled    : true,
                                                        space_type          : SpaceType::Armory,
                                                        ..Default::default()
                                                    },
        };
        return result;
    }
}

impl Armory {
    fn perform_special_action(&mut self) {
        let mut rng = rand::thread_rng();
        let item_type_roll = Uniform::from(0..=100);

        // Create a weapon
        let mut roll_result : u32 = item_type_roll.sample(&mut rng);

        if roll_result >= ARMORY_SWORD1000_DROP_PCT {
            self.space_data.pack.borrow_mut().add_item(Rc::new(SwordOfAThousandTruths::default()));
        }
        else if roll_result >= ARMORY_SCHIAVONA_DROP_PCT {
            self.space_data.pack.borrow_mut().add_item(Rc::new(Schiavona::default()));
        }
        else if roll_result >= ARMORY_FALCHION_DROP_PCT {
            self.space_data.pack.borrow_mut().add_item(Rc::new(Falchion::default()));
        }
        else { //if roll_result >= ARMORY_DIRK_DROP_PCT {
            self.space_data.pack.borrow_mut().add_item(Rc::new(Dirk::default()));
        }

        // Create a shield
        roll_result = item_type_roll.sample(&mut rng);

        if roll_result >= ARMORY_SCUTUM_DROP_PCT {
            self.space_data.pack.borrow_mut().add_item(Rc::new(Scutum::default()));
        }
        else if roll_result >= ARMORY_HEATER_DROP_PCT {
            self.space_data.pack.borrow_mut().add_item(Rc::new(Heater::default()));
        }
        else { //if roll_result >= ARMORY_BUCKLER_DROP_PCT {
            self.space_data.pack.borrow_mut().add_item(Rc::new(Buckler::default()));
        }

        println!(" The weapons rack is now open.");
        
        self.space_data.special_action_performed = true;
    }
}