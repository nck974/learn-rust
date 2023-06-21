use std::fmt::Debug;
pub trait Summary<T> {
    fn summarize(&self);
    fn get_values(&self) -> Vec<&T>;
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Debug> Summary<T> for Point<T>{
    fn summarize(&self){
        print!("Values: ");
        for value in self.get_values(){
            print!("({:?}) ", value);
        }
        println!();
    }
    fn get_values(&self) -> Vec<&T> {
        vec![&self.x, &self.y]
    }
}

fn main() {
    let point = Point { x: 5, y: 2 };
    let point_2 = Point { x: 5.0, y: 2.0};
    dbg!(&point);
    dbg!(&point_2);
    point.summarize();
    point_2.summarize();

    // LIFETIME

    // This does not work due to the lifetime of x:
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// Note the annotation for the lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}