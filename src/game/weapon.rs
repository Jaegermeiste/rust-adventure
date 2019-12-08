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
use traitcast::{/*traitcast,*/ TraitcastFrom};
use crate::game::item::*;

pub static	WEAPON_DEFAULT_ATTACK_POINTS    : u32 = 1;
pub static	WEAPON_DEFAULT_ATTACK_MODE_TEXT : &str = "attacks";

pub struct WeaponData {
    pub attack_points       : u32,
    pub attack_mode_text    : String,
}

impl Default for WeaponData {
    fn default() -> WeaponData {
        let data = WeaponData {
            attack_points       :   WEAPON_DEFAULT_ATTACK_POINTS,
            attack_mode_text    :   WEAPON_DEFAULT_ATTACK_MODE_TEXT.to_string(),
        };
        return data;
    }
}

pub trait Weapon: Item  + TraitcastFrom {
    fn  get_attack_points       (&self) -> u32;
    fn  get_attack_mode_text    (&self) -> String;
}

#[macro_export]
macro_rules! impl_Weapon { 
    ($T:ident) => {
        crate::impl_GameObject!($T);

        crate::impl_Item!($T);

/*
        // HACK: Work around language limitation regarding trait conversions/casting
        impl Item for $T {
            fn get_item_weight(&self) -> u32 {
                return self.item_data.weight; 
            }

            fn get_item_property(&self) -> ItemProperty {
                return self.item_data.property; 
            }

            fn get_item_type(&self) -> ItemType {
                return self.item_data.item_type; 
            }

            fn get_item_action_points  (&self) -> u32 {
                return self.get_attack_points();
            }

            fn get_item_action_text    (&self) -> String {
                return self.get_attack_mode_text();
            }

            fn as_weapon (&self) -> Option<Rc<&dyn Weapon>> {
                let option = Some(Rc::new(self as &dyn Weapon));

                return option;
            }

            fn as_shield (&self) -> Option<Rc<&dyn crate::game::shield::Shield>> {
                return Option::None;
            }
        }
*/

        impl Weapon for $T {
            fn get_attack_points(&self) -> u32 {
                 return self.weapon_data.attack_points; 
            }

            fn get_attack_mode_text(&self) -> String {
                return self.weapon_data.attack_mode_text.clone(); 
            }

        }

        // Type conversion
        traitcast!(struct $T: GameObject, Item, Weapon);
    }
}