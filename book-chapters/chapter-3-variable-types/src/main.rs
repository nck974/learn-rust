fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
fn main() {
    let guess: u32 = match "aa".parse() {
        Err(_) => 0,
        Ok(value) => value,
    };
    println!("{guess}");
    print_type_of(&guess);
    let guess = 23;
    print_type_of(&guess);
    let guess: f64 = 23 as f64 / 2 as f64;
    println!("{}", guess);
    print_type_of(&guess);

    // Tupple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    print_type_of(&tup);
    println!("{}", tup.0);
    let tup = (500, 6.4, 1);
    print_type_of(&tup);
    println!("{}", tup.0);

    // Array
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    print_type_of(&a);
    println!("{}", a[a.len() - 1]);

    // Function
    another_function(a[0]);
    expression();
    let value = return_value(25);
    println!("{}",value);

    // Flow control

    // if/else
    if value < 5 as f64 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = if true { 5 } else { 6 };

    println!("The value of number is: {number}");

    // Loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut count = 0;
    let value = 'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up 33;
            }
            remaining -= 1;
        }

        count += 1;
    };
    println!("The result is {value}");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
    for number in (1..=4).rev(){
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    // Proposed example:
    let common = "my true love gave to me ";
    let values = [
        "Partridge in a pear tree",
        "Turtle doves",
        "French hens",
        "Collie birds",
        "Golden rings",
        "Geese a-laying",
        "Swans a-swimming",
        "Maids a-milking",
        "Pipers piping",
        "Drummers drumming",
        "Lords a-leaping",
        "Ladies dancing", 
    ];
    for i in 1..=values.len(){
        print!("On the {} day of christmas ", i);
        println!("{common}");
        for j in 1..=i{
            let value = values[j-1];
            println!("{j} {value}")
        }
    }
    

}

fn another_function(x: i32) {
    println!("Another function. {}", x);
}

fn expression() {
    // This is an expression that yields to a value in y
    let y = {
        let x = 3;
        x + 1 // <---- As it is returning does not end with semicolon
    };

    println!("The value of y is: {y}");
}

fn return_value(x: i32) -> f64{
    return x as f64 * 5 as f64 / 25.0;
}
