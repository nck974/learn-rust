//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.


/// Adds 2 to the number given
/// 
/// # Examples:
/// 
/// ```rust
/// let x: i32 = 5;
/// let res = chapter_14_cargo::add_two(x);
/// 
/// assert_eq!(7, res);
/// ```
pub fn add_two(x: i32) -> i32{
    x + 2
}