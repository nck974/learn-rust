enum IpAddrKind {
    V4,
    V6,
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

fn main() {
    println!("Hello, world!");
    let ip = IpAddrKind::V4;
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // Enum with parameter
    let home = IpAddr::V4(String::from("127.0.0.1"));
    dbg!(&home);

    let loopback = IpAddr::V6(String::from("::1"));

    // None:
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    // Match

    let value = value_in_cents(Coin::Nickel);
    println!("{}", value);

    let mut count = 0;
    let coin = Coin::Nickel;
    // let coin = Coin::Quarter(String::from("Test"));
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25,
    }
}
