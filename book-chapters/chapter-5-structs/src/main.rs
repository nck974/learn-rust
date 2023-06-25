#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn calculate_area(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    println!("Hello, world!");
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.sign_in_count += 1;
    println!("{}", user1.active);
    println!("{}", user1.sign_in_count);
    println!("{}", user1.username);
    println!("{}", user1.email);
    println!("{:#?}", user1);

    // Rectangle area

    // DEBUG
    let width = 50.0;
    let height = 50.0;
    let rectangle = Rectangle {
        width,
        height: dbg!(height),
    };
    dbg!(&rectangle);
    
    // Function
    let area = calculate_rectangle_area(&rectangle);
    println!("{width} * {height} = {area}");    
    
    // Method
    let area = rectangle.calculate_area();
    println!("{width} * {height} = {area}");
}

fn calculate_rectangle_area(rectangle: &Rectangle) -> f64 {
    rectangle.width * rectangle.height
}
