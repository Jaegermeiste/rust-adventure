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
use crate::game::item::*;

pub static	SHIELD_DEFAULT_DEFENSE_POINTS       : u32   = 1;
pub static	SHIELD_DEFAULT_DEFENSE_MODE_TEXT    : &str  = "blocks";

pub struct ShieldData {
    pub defense_points       : u32,
    pub defense_mode_text    : String,
}

impl Default for ShieldData {
    fn default() -> ShieldData {
        let data = ShieldData {
            defense_points       :   SHIELD_DEFAULT_DEFENSE_POINTS,
            defense_mode_text    :   SHIELD_DEFAULT_DEFENSE_MODE_TEXT.to_string(),
        };
        return data;
    }
}

pub trait Shield: Item {
    fn  get_defense_points      (&self) -> u32;
    fn  get_defense_mode_text   (&self) -> String;
}

#[macro_export]
macro_rules! impl_Shield { 
    ($T:ident) => {
        impl Shield for $T {
            fn get_defense_points(&self) -> u32 {
                 return self.shield_data.defense_points; 
            }

            fn get_defense_mode_text(&self) -> String {
                return self.shield_data.defense_mode_text.clone(); 
            }
        }
    }
}