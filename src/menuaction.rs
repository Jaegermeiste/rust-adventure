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

pub enum MenuActionType
{
	MENUACTIONTYPE_NULL,
	MENUACTIONTYPE_SELECTOR,
	MENUACTIONTYPE_LAUNCH_MENU_PAGE,
	MENUACTIONTYPE_EXIT_MENU,
	MENUACTIONTYPE_BOOL,
	MENUACTIONTYPE_INT,
	MENUACTIONTYPE_INT_RANGE_UPPER_INCLUSIVE,
	MENUACTIONTYPE_INT_RANGE_UPPER_EXCLUSIVE,
	MENUACTIONTYPE_INT_LIST,
	MENUACTIONTYPE_STRING_LIST,
	MENUACTIONTYPE_STRING_LIST_OVERRIDE,
	MENUACTIONTYPE_STRING,
	MENUACTIONTYPE_MAX
}

pub struct MenuAction {
    action_text             : String,
    action_value            : i32,
    action_default_value    : i32,
    valid_value_list        : Vec<i32>,
    valid_value_string_list : Vec<String>,
    upper_bound_inclusive   : bool,
    action_type             : MenuActionType,
    min_action              : *const MenuAction,
    max_action              : *const MenuAction,
    default_value_action    : *const MenuAction,
    override_string         : String,
    overridden              : bool,
}

impl Default for MenuAction {
    fn default() -> MenuAction {
        MenuAction {
            action_text             : "Placeholder Text".to_string(),
            action_value            : -1,
            action_default_value    : -1,
            valid_value_list        : vec![i32::min_value(), i32::max_value()],
            valid_value_string_list : vec!["MinRange".to_string(), "MaxRange".to_string()],
            upper_bound_inclusive   : true,
            action_type             : MenuActionType::MENUACTIONTYPE_NULL,
            min_action              : std::ptr::null(),
            max_action              : std::ptr::null(),
            default_value_action    : std::ptr::null(),
            override_string         : "".to_string(),
            overridden              : false,
        }
    }
}

impl MenuAction {
    pub fn new(in_text : &str, in_type : MenuActionType) -> MenuAction {
        MenuAction { 
            action_text: in_text.to_string(),
            action_type: in_type,
            ..MenuAction::default()
            }
    }
}