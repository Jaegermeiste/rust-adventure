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

pub static	WEAPON_DEFAULT_ATTACK_POINTS    : u32 = 1;
pub static	WEAPON_DEFAULT_ATTACK_MODE_TEXT : &str = "attacks";

pub trait Weapon: Item {
    fn  get_attack_points       (&self) -> u32;
    fn  get_attack_mode_text    (&self) -> String;
}