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
use crate::game::items::item::*;
use crate::game::items::weapons::weapon::*;
use crate::game::items::shields::shield::*;

static	BACKPACK_NAME			: &str = "Satchel of Holding";
static  BACKPACK_FLAVOR_TEXT	: &str = " Heavily worn, this bag is made of a pliant leather of unknown origin.";
static  BACKPACK_MAX_WEIGHT		: u32 = 40;

pub struct Backpack {
    // GameObject
    game_object_data    : GameObjectData,
    
    items           :   Vec<Rc<dyn Item>>,

    weapon_index    :   usize,
    shield_index    :   usize,
}

crate::impl_GameObject!(Backpack);

impl Backpack {
    pub fn  new() -> Backpack {
        let pack = Backpack { 
            game_object_data    : GameObjectData    { name : String::from(BACKPACK_NAME), flavor_text : String::from(BACKPACK_FLAVOR_TEXT) },
            items               : Vec::new(),
            weapon_index        : usize::max_value(),
            shield_index        : usize::max_value(),
            };
        return pack;
    }

    pub fn  add_item(&mut self, item : Rc<dyn Item>) {
        if Rc::strong_count(&item) > 0 {
            self.items.push(item);
        }
    }

    pub fn  add_item_list (&mut self, item_list : Vec<Rc<dyn Item>>) {
        for item in item_list {
            self.add_item(item);

        }
    }

    pub fn  print_items (&self) {
        for (index, item_ref) in self.items.iter().enumerate() {
            let item = item_ref.as_ref();

            // Print basic info
            print!("     {}) {}     Weight: {}     ", (index + 1), item.name(), item.get_item_weight());
    
            // Print details as appropriate
            if item.get_item_type() == ItemType::Weapon {
                print!("Equippable as Weapon.");
            }
            else if item.get_item_type() == ItemType::Shield {
                print!("Equippable as Shield.");
            }
    
            if item.get_item_property() != ItemProperty::Droppable {
                print!(" Not transferable.");
            }
    
            // Print Flavor Text
            println!("     {}", item.flavor_text()); 
        }
    }

    pub fn  drop_item   (&mut self, index : usize) -> Option<Rc<dyn Item>> {
        let drop_item = self.items.get(index).unwrap();

        if drop_item.get_item_property() == ItemProperty::Droppable {
            let removed = self.items.remove(index as usize);	// Remove the element
    
            // Adjust indices
            if self.weapon_index > index {
                self.weapon_index -= 1;
            }
    
            if self.shield_index > index {
                self.shield_index -= 1;
            }

            return Option::Some(removed);
        }
        else {
            println!(" {} cannot be dropped or transferred.", drop_item.name());
        }
        return Option::None;
    }

    pub fn  droppable_item_list (&self) -> Vec<Rc<dyn Item>> {
        let mut drop_list : Vec<Rc<dyn Item>> = Vec::new();

        for item in self.items.iter() {
            if item.get_item_property() == ItemProperty::Droppable {
                drop_list.push(item.clone());
            }
        }

        return drop_list;
    }

    pub fn  drop_first_item_of_type   (&mut self, item_type : ItemType) -> Option<Rc<dyn Item>> {
        for (index, item) in self.items.iter().enumerate() {
            if item.get_item_type() == item_type {
                return self.drop_item(index);
            }
        }

        return Option::None;
    }

    pub fn  print_weapons (&self) {
        println!(" Weapons Inventory:");

        for (index, item) in self.items.iter().enumerate() {
            if item.get_item_type() == ItemType::Weapon {
                if traitcast::implements_trait::<dyn Item, dyn Weapon>(item.as_ref()) {
                    let weapon = traitcast::cast_ref::<dyn Item, dyn Weapon>(item.as_ref()).expect("Failed to unwrap Weapon");

                    println!("     {}) {}     Weight: {}     {} Equippable as Weapon with Attack = ", (index + 1), item.name(), item.get_item_weight(), weapon.attack_points());
                    println!("         {}", item.flavor_text());
                }
            }
        }
    }

    pub fn  set_weapon (&mut self, index : usize, verbose : bool) {
        // Verify valid index
        if index < self.items.len()
        {
            let item = self.items.get(index).unwrap();

            // Verify item at index is weapon
            if item.get_item_type() == ItemType::Weapon {
                if traitcast::implements_trait::<dyn Item, dyn Weapon>(item.as_ref()) {
                    let weapon = traitcast::cast_ref::<dyn Item, dyn Weapon>(item.as_ref()).expect("Failed to unwrap Weapon");

                    self.weapon_index = index;

                    if verbose {
                        println!(" Weapon set to {} with Attack = {}", weapon.name(), weapon.attack_points());
                    }
                }
            }
            else {
                println!(" Selected inventory Item cannot be equipped as a Weapon.");
            }
        }
        else {
            println!(" Invalid selection.");
        }
    }

    pub fn  current_weapon (&self) -> Option<Rc<&dyn Weapon>> {
        let mut option : Option<Rc<&dyn Weapon>> = Option::None;

        if (self.weapon_index < self.items.len()) && 
            (self.items.get(self.weapon_index as usize).unwrap().get_item_type() == ItemType::Weapon) {
            
                let item = self.items.get(self.weapon_index as usize).unwrap();
                if traitcast::implements_trait::<dyn Item, dyn Weapon>(item.as_ref()) {
                    let weapon = Rc::new(traitcast::cast_ref::<dyn Item, dyn Weapon>(item.as_ref()).expect("Failed to unwrap Weapon"));
                    option = Option::Some(weapon);
                }
        }

        if option.is_none() {
            println!(" No weapon currently selected.");
        }

        return option;
    }

    pub fn  print_shields (&self) {
        println!(" Shields Inventory:");

        for (index, item) in self.items.iter().enumerate() {
            if item.get_item_type() == ItemType::Shield {
                if traitcast::implements_trait::<dyn Item, dyn Shield>(item.as_ref()) {
                    let shield = traitcast::cast_ref::<dyn Item, dyn Shield>(item.as_ref()).expect("Failed to unwrap Shield");

                    println!("     {}) {}     Weight: {}     {} Equippable as Shield with Defense = ", (index + 1), item.name(), item.get_item_weight(), shield.defense_points());
                    println!("         {}", item.flavor_text());
                }
            }
        }
    }

    pub fn  set_shield (&mut self, index : usize, verbose : bool) {
        // Verify valid index
        if index < self.items.len()
        {
            let item = self.items.get(index as usize).unwrap();

            // Verify item at index is shield
            if item.get_item_type() == ItemType::Shield {
                if traitcast::implements_trait::<dyn Item, dyn Shield>(item.as_ref()) {
                    let shield = traitcast::cast_ref::<dyn Item, dyn Shield>(item.as_ref()).expect("Failed to unwrap Shield");

                    self.shield_index = index;

                    if verbose {
                        println!(" Shield set to {} with Defense = {}", shield.name(), shield.defense_points());
                    }
                }
            }
            else {
                println!(" Selected inventory Item cannot be equipped as a Shield.");
            }
        }
        else {
            println!(" Invalid selection.");
        }
    }

    pub fn  current_shield (&self) -> Option<Rc<&dyn Shield>> {
        let mut option : Option<Rc<&dyn Shield>> = Option::None;

        if (self.shield_index < self.items.len()) && 
            (self.items.get(self.shield_index as usize).unwrap().get_item_type() == ItemType::Shield) {
            
                let item = self.items.get(self.shield_index as usize).unwrap();
                if traitcast::implements_trait::<dyn Item, dyn Shield>(item.as_ref()) {
                    let shield = Rc::new(traitcast::cast_ref::<dyn Item, dyn Shield>(item.as_ref()).expect("Failed to unwrap Shield"));
                    option = Option::Some(shield);
                }
        }

        /*if option.is_none() {
            println!(" No shield currently selected.");
        }*/

        return option;
    }

    pub fn  overweight (&self) -> bool {
        let mut overweight = false;
        let mut weight = 0;

        // Sum the weight
        for item in self.items.iter() {
            weight += item.get_item_weight();
        }

        if weight > BACKPACK_MAX_WEIGHT {
            overweight = true;
        }

        return overweight;
    }

    pub fn  item_type_exists (&self, item_type : ItemType) -> bool {
        let mut found = false;

        for item in self.items.iter() {
            if item.get_item_type() == item_type {
                found = true;
                break;  // Early out
            }
        }

        return found;
    }

    pub fn  size (&self) -> u32{
        return self.items.len() as u32;
    }
}