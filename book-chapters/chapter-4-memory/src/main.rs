fn main() {
    let mut s = String::from("hello");
    println!("{s}");
    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    // Allocation issue when borrowing reference:
    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);    
    
    let s1 = String::from("hello");
    let s2 = using_string_without_move(&s1);

    println!("s1 = {}, s2 = {}", s1, s2);

    // Slices:
    let s = "Hello world";
    let var = first_word(&s);
    println!("{var}")

}

fn using_string_without_move(s: &String) -> String {
    s.to_string()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}