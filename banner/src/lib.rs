use std::{collections::HashMap, num::ParseFloatError};

// Define a struct to hold the flag information.
#[derive(PartialEq, Eq, Hash)] // This allows Flag to be used as a key in HashMap
pub struct Flag<'a> {
    pub short_hand: String, // A short-hand version of the flag (e.g., "-d")
    pub long_hand: String,  // A long-hand version of the flag (e.g., "--division")
    pub desc: &'a str,      // A description of what the flag does
}

// Implement associated functions for the Flag struct.
impl<'a> Flag<'a> {
    // This function creates a Flag struct with both short-hand and long-hand representations.
    pub fn opt_flag(name: &'a str, d: &'a str) -> Self {
        Self {
            short_hand: format!("-{}", name.chars().next().unwrap()), // Create short-hand flag (e.g., "-d")
            long_hand: format!("--{}", name),                        // Create long-hand flag (e.g., "--division")
            desc: d,                                                 // Set description
        }
    }
}

// Define a type alias for the callback function signature. It returns a Result with either a String or ParseFloatError.
pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

// Define the FlagsHandler struct that will manage the flags and their associated callback functions.
pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>, // HashMap to store flags and their associated functions
}

// Implement associated functions for the FlagsHandler struct.
impl FlagsHandler {
    // Adds a flag and its associated callback function to the HashMap
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        // Insert both the short-hand and long-hand flags with the same callback function
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
    }

    // Executes the callback function associated with a flag and returns the result.
    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        // Look up the callback function based on the flag input
        let f = self.flags[input];
        // Execute the callback with the arguments and return the result, or handle errors
        f(argv[0], argv[1]).map_err(|e| e.to_string()) // Convert the ParseFloatError to a String error
    }
}

// Define the `div` function, which performs division on two strings and returns the result as a string.
pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    // Try to parse the strings as f64 and return the division result as a string
    Ok((a.parse::<f64>()? / b.parse::<f64>()?).to_string())
}

// Define the `rem` function, which calculates the remainder of dividing two strings and returns the result as a string.
pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    // Try to parse the strings as f64 and return the remainder as a string
    Ok((a.parse::<f64>()? % b.parse::<f64>()?).to_string())
}
