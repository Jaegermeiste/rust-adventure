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

static PLAYER_NAME          : &str = "Chain";

static PLAYER_FLAVOR_TEXT	: &str = " Definitely not Zelda.";

static PLAYER_XP_LEVEL_1	: u32 = 100;	// Rats
static PLAYER_XP_LEVEL_2	: u32 = 500;	// Spectre
static PLAYER_XP_LEVEL_3	: u32 = 1000;	// Skeleton
static PLAYER_XP_LEVEL_4	: u32 = 3000;	// Demogorgon

static PLAYER_MAX_HEALTH	: u32 = 100;