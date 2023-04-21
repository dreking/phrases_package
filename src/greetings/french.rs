//! French greetings
//!
//! # Examples
//! ```
//! use phrases::greetings::french;
//! println!("{}", french::hello());
//! println!("{}", french::goodbye());
//! ```

/// Returns a French greeting
pub fn hello() -> String {
    "Bonjour!".to_string()
}

/// Returns a French farewell
pub fn goodbye() -> String {
    "Au revoir!".to_string()
}
