//! This module contains English phrases
//! 
//! # Examples
//! ```
//! let username = "John";
//! println!("{}, {}", 
//!     phrases::greetings::english::hello(), 
//!     username);
//! ```
pub fn hello() -> String { "Hello".to_string() }
pub fn goodbye() -> String { "Goodbye".to_string() }