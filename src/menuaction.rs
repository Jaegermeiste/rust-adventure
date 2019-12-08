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
use std::cell::Cell;
//use num_derive::FromPrimitive;    
//use num_traits::FromPrimitive;

#[derive(/*FromPrimitive,*/ PartialOrd, PartialEq, Eq, Clone, Debug)]
pub enum MenuActionType {
	Null,
	Selector,
	LaunchMenuPage,
	ExitMenu,
	Bool,
	Int,
	IntRangeUpperInclusive,
	IntRangeUpperExclusive,
	IntList,
	StringList,
	StringListOverride,
	String,
	Max
}

pub struct MenuAction {
    action_text             : String,
    action_value            : Cell<i32>,
    action_default_value    : i32,
    valid_value_list        : Vec<i32>,
    valid_value_string_list : Vec<String>,
    upper_bound_inclusive   : bool,
    action_type             : MenuActionType,
    min_action              : Option<&'static mut MenuAction>,
    max_action              : Option<&'static mut MenuAction>,
    default_value_action    : Option<&'static mut MenuAction>,
    override_string         : String,
    overridden              : bool,
}

impl Default for MenuAction {
    fn default() -> MenuAction {
        MenuAction {
            action_text             : "Placeholder Text".to_string(),
            action_value            : Cell::new(-1),
            action_default_value    : -1,
            valid_value_list        : vec![i32::min_value(), i32::max_value()],
            valid_value_string_list : vec!["min_range".to_string(), "max_range".to_string()],
            upper_bound_inclusive   : true,
            action_type             : MenuActionType::Null,
            min_action              : None,
            max_action              : None,
            default_value_action    : None,
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

    pub fn get_string(&self) -> String {
        return self.action_text.clone();
    }

    pub fn get_value_int(&self) -> i32 {
        return self.action_value.clone().get();
    }

    pub fn get_value_bool(&self) -> bool {
        if self.action_value.get() >= 1 {
            return true;
        }

        return false;
    }

    pub fn get_value_string(&self) -> String {
        let mut index : usize = 0;

        if self.overridden {
            return self.override_string.clone();
        }
        else if self.action_type == MenuActionType::StringListOverride {
            let diff = self.default_value_action.as_ref().unwrap().get_value_int() - self.default_value_action.as_ref().unwrap().get_min_range_value();

            if diff < 0 {
                index = 0;
            }
            else {
                index = diff as usize;
            }
        }
        else {
            for i in 0..self.valid_value_list.len() {
                if *(self.valid_value_list.get(i).expect("Failed to get value.")) == self.action_value.get() {
                    index = i;
                }
            }
        }

        let mut return_string : String = String::new();
        return_string.clone_from(self.valid_value_string_list.get(index).expect("Failed to get element."));
        return return_string;
    }

    pub fn get_default_value(&self) -> i32 {
        return self.action_default_value.clone();
    }

    pub fn get_default_value_string(&self) -> String {
        let mut index : usize = 0;

        if !self.default_value_action.is_none() {
            let diff = self.default_value_action.as_ref().unwrap().get_value_int() - self.default_value_action.as_ref().unwrap().get_min_range_value();

            if diff < 0 {
                index = 0;
            }
            else {
                index = diff as usize;
            }
        }
        else {
            for i in 0..self.valid_value_list.len() {
                if *(self.valid_value_list.get(i).expect("Failed to get value.")) == self.action_default_value {
                    index = i;
                }
            }
        }

        let mut return_string : String = String::new();
        return_string.clone_from(self.valid_value_string_list.get(index).clone().expect("Failed to find value string."));
        return return_string;
    }

    pub fn get_min_range_value(&self) -> i32 {
        let return_value;

        if self.min_action.is_none() == false {
            // Since the pointer is not NULL, assume we need to use the referenced value
            return_value = self.min_action.as_ref().unwrap().get_value_int();
        }
        else {
            return_value = *(self.valid_value_list.get(0).expect("Vector access failed."));
        }

        return return_value;
    }

    pub fn get_max_range_value(&self) -> i32 {
        let return_value;

        if self.max_action.is_none() == false {
            // Since the pointer is not NULL, assume we need to use the referenced value
            return_value = self.max_action.as_ref().unwrap().get_value_int();
        }
        else {
            return_value = *(self.valid_value_list.get(self.valid_value_list.len() - 1).expect("Vector access failed."));
        }

        return return_value;
    }

    pub fn get_type(&self) -> MenuActionType {
        return self.action_type.clone();
    }

    pub fn get_allowed_value_list(&self) -> Vec<i32> {
        return self.valid_value_list.clone();
    }

    pub fn get_allowed_value_strings(&self) -> Vec<String> {
        return self.valid_value_string_list.clone();
    }

    pub fn set_value_i32(&self, in_value : i32) -> bool {
        let mut success : bool = false;

        if self.upper_bound_inclusive == true
        {
            // in_value <= upper
            if (in_value >= self.get_min_range_value()) && (in_value <= self.get_max_range_value())
            {
                success = true;
            }
        }
        else
        {
            // in_value < upper
            if (in_value >= self.get_min_range_value()) && (in_value < self.get_max_range_value())
            {
                success = true;
            }
        }
    
        if success == true {
            self.action_value.set(in_value);
        }
        else {
            println!(" ERROR: Value {} provided to MenuAction::set_value is outside the legal interval {}-{}, inclusive. Value not set.", in_value, self.get_min_range_value(), self.get_max_range_value());
        }
    
        return success;
    }

    pub fn set_value_string<S: Into<String>>(&mut self, in_string : S) -> bool {
	let mut success : bool = false;

	if self.action_type == MenuActionType::String {
		// Default is value 0, custom is value 1
		if self.valid_value_string_list.len() > 1 {
            let value = self.valid_value_string_list.get_mut(1).expect("Failed to get string.");
            value.clone_from(&in_string.into());
			self.action_value.set(1);
		}
		else
		{
			self.valid_value_string_list.push(in_string.into());
			self.action_value.set(1);
		}
		success = true;
	}
	else if self.action_type == MenuActionType::StringListOverride {
		self.override_string = in_string.into();
		self.overridden = true;
		success = true;
	}
	else
	{
		println!(" WARNING: Value {} provided to MenuAction::SetValue is invalid for MenuActionType {}. Value not set.", in_string.into(), self.action_type.clone() as u32);
	}

	return success;
}

    pub fn on_selected(&self) -> (MenuActionType, i32) {
        (self.action_type.clone(), self.action_value.get().clone())
    }

    pub fn reset(&self) {
        self.set_value_i32(self.action_default_value);
    }
}