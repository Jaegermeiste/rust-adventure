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

pub struct LocationData {
    pub x_coord     : u32,
    pub y_coord     : u32,
}

impl Default for LocationData {
    fn default() -> LocationData {
        let data = LocationData {
            x_coord                     :   0,
            y_coord                     :   0,
        };

        return data;
    }
}

pub trait Locatable: GameObject + TraitcastFrom {
    fn set_x_coord          (&mut self, coord : u32);
    fn set_y_coord          (&mut self, coord : u32);
    fn set_coords           (&mut self, x : u32, y : u32);

    fn x_coord              (&self)               -> u32;
    fn y_coord              (&self)               -> u32;
}

#[macro_export]
macro_rules! impl_Locatable { 
    ($T:ident) => {
        impl Locatable for $T {
            fn set_x_coord(&mut self, coord : u32) {
                 self.location_data.x_coord = coord; 
            }

            fn set_y_coord(&mut self, coord : u32) {
                self.location_data.y_coord = coord; 
            }

            fn set_coords(&mut self, x : u32, y : u32) {
                self.location_data.x_coord = x; 
                self.location_data.y_coord = y; 
            }

            fn x_coord(&self) -> u32 {
                return self.location_data.x_coord; 
            }

            fn y_coord(&self) -> u32 {
                return self.location_data.y_coord; 
            }
        }
    }
}