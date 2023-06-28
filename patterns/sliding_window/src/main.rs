//! Exercise:
//! Given an array of integers and a window size, implement a function that finds the maximum sum within
//! a sliding window of the given size. Return a new array containing the maximum sum for each window
//! position.
//!
//! Input:
//!     An array of integers: numbers (e.g., [4, 2, 1, 7, 8, 1, 2, 8, 1, 0])
//!     Window size: k (e.g., 3)
//!
//! Expected Output:
//! The expected output is a new array that contains the maximum sum for each sliding window position.
//!
//! For the provided input, the expected output would be: [7, 10, 16, 16, 11, 11, 11]
//!

use sliding_window::calculate_max_number;
fn main() {
    let numbers: Vec<i32> = vec![4, 2, 1, 7, 8, 1, 2, 8, 1, 0];
    let window_size: usize = 3;

    let res = calculate_max_number(numbers, window_size);
    dbg!("{}", res);
}
