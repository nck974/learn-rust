
use chapter_7_packages::eat_at_restaurant;

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);

    eat_at_restaurant();
}