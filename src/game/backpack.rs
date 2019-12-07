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
use crate::game::item::*;
use crate::game::weapon::*;
use crate::game::rattooth::*;
use crate::game::shield::*;
use crate::game::buckler::*;

static	BACKPACK_NAME			: &str = "Satchel of Holding";
static  BACKPACK_FLAVOR_TEXT	: &str = " Heavily worn, this bag is made of a pliant leather of unknown origin.";
static  BACKPACK_MAX_WEIGHT		: u32 = 40;

pub struct Backpack {
    // GameObject
    game_object_data    : GameObjectData,
    
    items           :   Vec<Rc<dyn Item>>,

    weapon_index    :   u32,
    shield_index    :   u32,
}

crate::impl_GameObject!(Backpack);

impl Backpack {
    pub fn  new() -> Backpack {
        let pack = Backpack { 
            game_object_data    : GameObjectData    { name : String::from(BACKPACK_NAME), flavor_text : String::from(BACKPACK_FLAVOR_TEXT) },
            items               : Vec::new(),
            weapon_index        : u32::max_value(),
            shield_index        : u32::max_value(),
            };
        return pack;
    }

    pub fn  add_item(&mut self, item : Rc<dyn Item>) {

    }

    pub fn  add_item_list (&mut self, item : Vec<Rc<dyn Item>>) {

    }

    pub fn  print_items (&self) {

    }

    pub fn  drop_item   (&self, index : u32) -> Rc<dyn Item> {
        return Rc::new(RatTooth::default());        // FIXME
    }

    pub fn  get_droppable_item_list (&self) -> Vec<Rc<dyn Item>> {
        return self.items.clone();       // FIXME
    }

    pub fn  drop_first_item_of_type   (&self, item_type : ItemType) -> Rc<dyn Item> {
        return Rc::new(RatTooth::default());        // FIXME
    }

    pub fn  print_weapons (&self) {

    }

    pub fn  set_weapon (&self, index : u32, verbose : bool) {

    }

    pub fn  get_current_weapon (&self) -> Rc<dyn Weapon> {
        return Rc::new(RatTooth::default());        // FIXME
    }

    pub fn  print_shields (&self) {

    }

    pub fn  set_shield (&self, index : u32, verbose : bool) {

    }

    pub fn  get_current_shield (&self) -> Rc<dyn Shield> {
        return Rc::new(Buckler::default());        // FIXME
    }

    pub fn  is_overweight (&self) -> bool {
        return false;       // FIXME
    }

    pub fn  item_type_exists (&self, item_type : ItemType) -> bool {
        return false;       // FIXME
    }

    pub fn  get_size (&self) -> u32{
        return 0;           // FIXME
    }
}