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

pub struct GameObjectData {
    pub name        : String,
    pub flavor_text : String,
}

impl Default for GameObjectData {
    fn default() -> GameObjectData {
        let data = GameObjectData {
            name        :   "Default Name".to_string(),
            flavor_text :   "Default Flavor Text".to_string(),
        };
        return data;
    }
}

pub trait GameObject: TraitcastFrom {
    fn name         (&self) -> String;
    fn flavor_text  (&self) -> String;
}

#[macro_export]
macro_rules! impl_GameObject { 
    ($T:ident) => {
        impl GameObject for $T {
            fn name(&self) -> String {
                 return self.game_object_data.name.clone(); 
            }

            fn flavor_text(&self) -> String {
                return self.game_object_data.flavor_text.clone(); 
           }
        }
    }
}