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
use crate::game::gameobject::*;

enum ItemProperty {
	None,
	Droppable,
	Max
}

enum ItemType {
	Default,
	Weapon,
	Shield,
	Health,
	ThroneRoomKey,
	GameWin,
	Max
}

pub static	DEFAULT_ITEM_WEIGHT : u32 =	1;

pub trait Item: GameObject {
    fn  get_item_weight     (&self) -> u32;
    fn  get_item_property   (&self) -> ItemProperty;
    fn  get_item_type       (&self) -> ItemType;
}