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
use std::io::{stdout, Write, Read};
use std::time::Duration;
use std::cell::Cell;
extern crate crossterm;
use crossterm::{
    execute,
    //input::{input, KeyEvent},
    //screen::RawScreen,
    terminal::{self, Clear, ClearType, SetSize, EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
    event::{self, poll, read, Event, KeyCode, KeyEvent, DisableMouseCapture, EnableMouseCapture},
    cursor::{EnableBlinking, Show},
    Result,
};
extern crate lexical;

//pub static DEFAULT_CANCEL_VAL : char =	0x04 as char;   /* EOT */
pub static DEFAULT_CANCEL_VAL : u8 = 0x04; //0x04_u8.into();
pub static MAX_STR_LEN : u32 =	std::u8::MAX as u32;   /* 255 */

#[derive(PartialEq, Eq, Clone)] 
pub enum InputNumericType
{
	Double,
	SignedInteger,
    UnsignedInteger
}

pub struct Input {
    canceled    : Cell<bool>,
}

impl  Default for Input {
    fn default() -> Input {
        let input = Input {
            canceled    : Cell::new(false),
        };
        return input;
    }
}

impl  Input {
    pub fn new() -> Rc<Input> {
        let input = Input { 
            canceled    : Cell::new(false), 
            };
        return Rc::new(input);
    }

    pub fn canceled(&self) -> bool {
        self.canceled.get()
    }

    // CrossTerm replacement functions
    /*pub fn read_char(&self) -> Result<char> {
        loop {
            if let Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) = event::read()?
            {
                return Ok(c);
            }
        }
    }*/

    pub fn read_char(&self) -> Result<char> {  
        let in_char = std::io::stdin().bytes().next().map(|b| b.unwrap() as char).unwrap();

            return Ok(in_char);
    }

    pub fn peek_char(&self) -> Result<char> {

        loop {
            match std::io::stdin().by_ref().bytes().peekable().peek() {
                Some(&Ok(ch)) => {
                    match ch {
                        _ => return Ok(ch as char),
                    }
                }
                Some(&Err(_)) => panic!(),
                None => break,
            }
        }
        
        return Ok('\0');
    }

    /*pub fn read_line(&self) -> Result<String> {
        let mut line = String::new();

        line.clear();

        while let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Enter => {
                    break;
                },
                KeyCode::Char(c) => {
                    line.push(c);
                },
                _ => {}
            }
        }
    
        return Ok(line);
    }*/

    fn read_line(&self) -> Result<String> {
        let mut rv = String::new();
        std::io::stdin().read_line(&mut rv)?;
        let len = rv.trim_end_matches(&['\r', '\n'][..]).len();
        rv.truncate(len);
        Ok(rv)
    }

    pub fn read_sync(&self) -> Result<Event> {
        return event::read();
    }

    pub fn read_async(&self) -> Result<Event> {
        if poll(Duration::from_millis(0))? {
            // Guaranteed that `read()` wont block if `poll` returns `Ok(true)`
            let event = event::read()?;

            return Ok(event);
        }
    
        return Err(crossterm::ErrorKind::__Nonexhaustive);
    }

    pub fn enable_mouse_mode(&self) -> Result<()> {
        execute!(std::io::stdout(), event::EnableMouseCapture)
    }

    pub fn disable_mouse_mode(&self) -> Result<()> {
        execute!(std::io::stdout(), event::DisableMouseCapture)
    }

    // End Crossterm replacements

    pub fn get_keypress_from_console(&self) -> char
    {
        let mut input_value : char = '\0';
        let mut flag_input_valid : bool = false;
        self.canceled.set(false);

        self.disable_mouse_mode();

        enable_raw_mode();

        while flag_input_valid == false
        {
            match self.read_char() {
                Ok(c) => input_value = c,
                Err(e) => println!("Read Error: {}", e),
            }

            if input_value != '\0'
            {
                // Set flag_input_valid to true so that loop ends on next go-around
                flag_input_valid = true;
            }
            else
            {
                // Display a message indicating to the user the invalid input, and to try again
                println!(" Invalid input. Please try again. Valid input is any keyboard character.");

                // Sanity check: Set flag_input_valid to false to ensure that loop continues
                flag_input_valid = false;
            }
        }

        disable_raw_mode();

        return input_value;
    }

    pub fn get_boolean_value_from_console(&self) -> bool {
        let mut flag_input_valid = false;
        let mut return_value : bool = false;

        while flag_input_valid == false
        {
            let mut input_value : char = self.get_keypress_from_console();
            if (input_value == 'y') || (input_value == 'Y') || (input_value == '1')
            {
                // Set flag_input_valid to true so that loop ends on next go-around
                flag_input_valid = true;
                return_value = true;
            }
            else if (input_value == 'n') || (input_value == 'N') || (input_value == '0')
            {
                // Set flag_input_valid to true so that loop ends on next go-around
                flag_input_valid = true;
                return_value = false;
            }
            else
            {
                // Display a message indicating to the user the invalid input, and to try again
                println!(" Invalid input. Please try again. Valid inputs are y, Y, n, N, and 0 or 1.");

                // Sanity check: Set flag_input_valid to false to ensure that loop continues
                flag_input_valid = false;
            }
        }

        return return_value;
    }

    pub fn get_string_from_console_bounds(&self, min_chars: u32, max_chars: u32) -> String {
        let mut input_value : String = "".to_string();
        let mut flag_input_valid : bool = false;
        //let input = crossterm::input::input();
        self.canceled.set(false);

        while flag_input_valid == false {
            input_value = "".to_string();

            match self.read_line() {
                Ok(s) => input_value = s,
                Err(e) => println!("Read Error: {}", e),
            }

            if (((input_value.len() as u32) - 1) >= min_chars) && (((input_value.len() as u32) - 1) <= max_chars) {
                // Set flag_input_valid to true so that loop ends on next go-around
                flag_input_valid = true;
            }
            else {
                // Display a message indicating to the user the invalid input, and to try again
                println!(" Invalid input. Please try again. Valid input is a minimum {0}, maximum {1}-character string.", min_chars, max_chars);

                // Sanity check: Set flag_input_valid to false to ensure that loop continues
                flag_input_valid = false;
            }
        }

        return input_value;
    }

    pub fn get_string_from_console(&self) -> String {
        return self.get_string_from_console_bounds(0, MAX_STR_LEN);
    }

    pub fn get_double_value_from_console(&self, mut lower_bound : f64, mut upper_bound : f64, numeric_type : InputNumericType, cancel_value: u8) -> f64 {
        let mut	input_value :f64		= std::f64::NAN;
        let mut flag_input_valid : bool	= false;
        let mut test_char : u8		= 0;     // NULL

        //self.disable_mouse_mode();

        // Reset flag
        self.canceled.set(false);

         // Ensure bounds are valid for input type
        match numeric_type.clone()
        {
            InputNumericType::UnsignedInteger => {
                if lower_bound < (std::u32::MIN as f64) {
                    lower_bound = std::u32::MIN as f64;
                }
                else if upper_bound > (std::u32::MAX as f64) {
                    upper_bound = std::u32::MAX as f64;
                }
            }
            InputNumericType::SignedInteger => {
                if lower_bound < (std::i32::MIN as f64) {
                    lower_bound = std::i32::MIN as f64;
                }
                else if upper_bound > (std::i32::MAX as f64) {
                    upper_bound = std::i32::MAX as f64;
                }
            }
            InputNumericType::Double => (),
            _ => (),
        }

        // Ensure cursor is shown
        let _ = execute!(
            stdout(),
            EnableBlinking,
            Show
        );

        while (flag_input_valid == false) && (self.canceled.get() == false)
        {
            // Grab a test char from stdin
            let var = self.peek_char();
            test_char = var.ok().unwrap() as u8;
            
            if self.canceled.get() == false {
                let mut input_line : String = "".to_string();
                input_line.clear();

                // if the test char was numeric, push it into the string so we don't lose it
                if ((test_char as char >= '0') && (test_char as char <= '9')) || (test_char as char == '.') {
                    input_line.push(test_char as char);
                }
                
                match self.read_line() {
                    Ok(s) => {
                        input_line.push_str(s.as_str());
                        //println!(" DEBUG: Input line: {}", input_line);
                    },
                    Err(e) => println!("Read Error: {}", e),
                }

                let parsed_result = lexical::parse::<f64, _>(input_line.as_str());

                let mut passed_parsing = false;
                match parsed_result {
                    Ok(v) => {
                        input_value = v;
                        passed_parsing = true;
                    },
                    Err(v) => passed_parsing = false,
                    _ => passed_parsing = false,
                }

                if  passed_parsing &&
                    (input_value != std::f64::NAN) && 
                    (input_value >= lower_bound) && 
                    (input_value <= upper_bound) &&
                    ((numeric_type == InputNumericType::Double) || (input_value.round() == input_value)) {
                    // Set flag_input_valid to true so that loop ends on next go-around
                    flag_input_valid = true;
                }
                else {
                    // Display a message indicating to the user the invalid input, and to try again
                    match numeric_type.clone()
                    {
                        InputNumericType::UnsignedInteger => {
                            println!(" Invalid input. Please try again. Valid input is an unsigned integer number ranging from {0:.} to {1:.}", lower_bound as u32, upper_bound as u32);
                        }
                        InputNumericType::SignedInteger => {
                            println!(" Invalid input. Please try again. Valid input is a signed integer number ranging from {0:.} to {1:.}", lower_bound as i32, upper_bound as i32);
                        }
                        _ => {
                            println!(" Invalid input. Please try again. Valid input is a floating-point number ranging from {0} to {1}", lower_bound, upper_bound);
                        }
                    }

                    // Sanity check: Set flag_input_valid to false to ensure that loop continues
                    flag_input_valid = false;
                }
            }
        }

        return input_value;
    }

    pub fn get_double_value_from_console_list (&self, mut allowed_value_list : Vec<f64>, numeric_type : InputNumericType) -> f64 {
        let mut	input_value :f64		= std::f64::NAN;
        let mut flag_input_valid : bool	= false;

        // Ensure bounds are valid for input type
        for element in allowed_value_list.iter_mut() {
            match numeric_type.clone() {
                InputNumericType::UnsignedInteger => {
                    if *element < (std::u32::MIN as f64) {
                        *element = std::u32::MIN as f64;
                    }
                    else if *element > (std::u32::MAX as f64) {
                        *element = std::u32::MAX as f64;
                    }
                }
                InputNumericType::SignedInteger => {
                    if *element < (std::i32::MIN as f64) {
                        *element = std::i32::MIN as f64;
                    }
                    else if *element > (std::i32::MAX as f64) {
                        *element = std::i32::MAX as f64;
                    }
                }
                InputNumericType::Double => (),
                _ => (),
            }
        }

        while flag_input_valid == false {
            let mut input_line : String = "".to_string();
            input_line.clear();
            input_value = std::f64::NAN;

            match self.read_line() {
                Ok(s) => input_line = s,
                Err(e) => println!("Read Error: {}", e),
            }

            //input_value = input_line.parse::<f64>().unwrap();
            let parsed_result = lexical::parse::<f64, _>(input_line.as_str());

            let mut passed_parsing = false;
            match parsed_result {
                Ok(v) => {
                    input_value = v;
                    passed_parsing = true;
                },
                Err(v) => passed_parsing = false,
                _ => passed_parsing = false,
            }

            if passed_parsing && 
            (input_value != std::f64::NAN) {
                // Input was of the right type, now check it against the allowed value list
                for element in allowed_value_list.iter() {
                    if input_value == *element {
                        // We found the value in the list, so set flag_input_valid to true so that loop ends on next go-around
                        flag_input_valid = true;
                        
                        // Terminate loop early
                        break;
                    }
                }
            }

            if flag_input_valid == false {
                // Display a message indicating to the user the invalid input, and to try again
                let mut error_msg : String = " Invalid input. Please try again. Valid inputs are any of the following: ".to_string();

                let mut counter : u32 = 0;      // Counter to find the last element in the iterator
                for element in allowed_value_list.iter() {

                    let mut trail_char : String = ", ".to_string();

                    if counter == ((allowed_value_list.len() as u32) - 1) {
                        // End of the list, output a period and endl
                        trail_char = ".".to_string();
                    }

                    match numeric_type.clone()
                    {
                        InputNumericType::UnsignedInteger => {
                            error_msg = format!("{} {:.}{}", error_msg, element, trail_char);
                        }
                        InputNumericType::SignedInteger => {
                            error_msg = format!("{} {:.}{}", error_msg, element, trail_char);
                        }
                        _ => {
                            error_msg = format!("{} {:.}{}", error_msg, element, trail_char);
                        }
                    }

                    // Increment loop counter
                    counter += 1;
                }

                // Output concat'd message
                println!("{}", error_msg);

                // Sanity check: Set flag_input_valid to false to ensure that loop continues
                flag_input_valid = false;
            }
        }

        return input_value;
    }

    pub fn get_signed_integer_value_from_console_range(&self, lower_bound: i32, upper_bound: i32, cancel_val: u8) -> i32 {
	    return self.get_double_value_from_console(lower_bound as f64, upper_bound as f64, InputNumericType::SignedInteger, cancel_val).round() as i32;
    }

    pub fn get_signed_integer_value_from_console(&self, cancel_val: u8) -> i32 {
	    return self.get_double_value_from_console(std::i32::MIN as f64, std::i32::MAX as f64, InputNumericType::SignedInteger, cancel_val).round() as i32;
    }

    pub fn get_signed_integer_value_from_console_list (&self, allowed_value_list: Vec<i32>) -> i32 {
        let mut double_value_list: Vec<f64> = Vec::with_capacity(allowed_value_list.len());

        for element in allowed_value_list.iter() {
            double_value_list.push(element.clone() as f64);
        }

        return self.get_double_value_from_console_list(double_value_list, InputNumericType::SignedInteger).round() as i32;
    }

    pub fn get_unsigned_integer_value_from_console_range(&self, lower_bound: u32, upper_bound: u32, cancel_val: u8) -> u32 {
	    return self.get_double_value_from_console(lower_bound as f64, upper_bound as f64, InputNumericType::UnsignedInteger, cancel_val).round() as u32;
    }

    pub fn get_unsigned_integer_value_from_console(&self, cancel_val: u8) -> u32 {
	    return self.get_double_value_from_console(std::i32::MIN as f64, std::i32::MAX as f64, InputNumericType::UnsignedInteger, cancel_val).round() as u32;
    }

    pub fn get_unsigned_integer_value_from_console_list (&self, allowed_value_list: Vec<u32>) -> u32 {
        let mut double_value_list: Vec<f64> = Vec::with_capacity(allowed_value_list.len());

        for element in allowed_value_list.iter() {
            double_value_list.push(element.clone() as f64);
        }

        return self.get_double_value_from_console_list(double_value_list, InputNumericType::SignedInteger).round() as u32;
    }
}