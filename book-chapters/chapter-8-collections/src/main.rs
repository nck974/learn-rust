use regex::Regex;
use std::collections::HashMap;
use std::io;

fn main() {
    // vector
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(3);
    v.push(4);
    for x in v {
        println!("{}", x);
    }

    // Shortcut with macro
    let mut v = vec![1, 2, 3];
    v.push(2);
    for x in &v {
        println!("{}", x);
    }

    // Getting element:
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    // Get syntax prevents panic
    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    println!("{does_not_exist:?}");

    // Iterate over a vector
    let vec = vec![23, 42, 23];
    for v in &vec {
        println!("{v}");
    }

    // With changes
    let mut vec = vec![23, 42, 23];
    for v in &mut vec {
        *v = *v + 1;
        println!("{v}");
    }

    // Enum which has just one type for the vector:
    #[derive(Debug)]
    enum MyEnum {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        MyEnum::Int(23),
        MyEnum::Float(23.0),
        MyEnum::Text(String::from("test")),
    ];
    for v in &row {
        dbg!(v);
    }

    // String slice
    let s_literal = "test string";
    println!("{s_literal}");

    // String
    let mut s = s_literal.to_string();
    s.push_str(" bar");
    s.push(' '); // Push is single character
    dbg!(s);

    let s_from_slice = String::from("initial contents");
    dbg!(s_from_slice);

    // utf8 in both slices and strings
    let hello = String::from("á");
    dbg!(hello);

    // Concat strings:
    let s1 = String::from("slice 1 ");
    let s2 = String::from("slice 2 ");
    let s3 = s1 + &s2; // it has to be borrowed, and s1 will not longer be available
    dbg!(s3);

    // HASHMAP
    let mut hash = HashMap::new();
    hash.insert(String::from("test"), 2);
    dbg!(&hash);
    println!("{:?}", hash.get_key_value(&String::from("test")));

    // Accessing data in the hashmap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let score = scores.get(&String::from("Blue")).copied().unwrap_or(0);
    dbg!(score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Exercise:
    let v = vec![1, 1, 2, 2, 2, 3, 4, 5, 6, 6, 6, 6, 7, 8];

    let length = v.len();
    let median = v[length / 2];
    dbg!(median);

    let mut count = HashMap::new();
    for x in &v {
        *count.entry(x).or_insert(0) += 1;
    }
    let mode = count.iter().max_by_key(|&(_, v)| v);
    dbg!(mode);
    dbg!(count);

    // Exercise
    // Convert strings to pig latin. The first consonant of each word is moved to the end of the
    // word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have
    // “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about
    // UTF-8 encoding!
    dbg!(pig_latin(String::from("first")));
    dbg!(pig_latin(String::from("apple")));

    // Exercise
    // Using a hash map and vectors, create a text interface to allow a user to add employee names
    // to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.”
    // Then let the user retrieve a list of all people in a department or all people in the company
    //  by department, sorted alphabetically.
    let roles = get_roles();
    dbg!(roles);
}

fn pig_latin(mut s: String) -> String {
    let vocals = ['a', 'e', 'i', 'o', 'u'];
    if let Some(first_char) = s.chars().next() {
        if vocals.contains(&first_char) {
            s.push_str("-hay");
        } else {
            s.remove(0);
            s.push(first_char);
            s.push_str("-fay");
        }
        return s;
    } else {
        return s;
    }
}

fn get_roles() -> HashMap<String, Vec<String>> {
    let mut roles: HashMap<String, Vec<String>> = HashMap::new();

    println!("Please write a command like 'Add <name> to <department>':");
    println!("Write x to exit:");
    let mut input_string = String::new();
    while input_string.trim() != "x" {
        input_string.clear();
        io::stdin().read_line(&mut input_string).unwrap();

        if input_string == "x" {
            break;
        }

        let re = Regex::new(r"Add (?P<user>\D+) to (?P<department>\D+)").unwrap();
        if let Some(cap) = re.captures_iter(&input_string).next() {
            let user: &str = &cap["user"].trim();
            let department: &str = &cap["department"].trim();
            println!("User: '{}' - Department: '{}'", user, department);
            roles
                .entry(String::from(department))
                .and_modify(|e| e.push(String::from(user)))
                .or_insert(vec![String::from(user)]);
            continue;
        }

        println!("Invalid command '{}'", input_string);
    }
    return roles;
}
