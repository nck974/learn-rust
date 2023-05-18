fn main() {
    // vector
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(3);
    v.push(4);
    for x in v{
        println!("{}",x);
    }

    // Shortcut with macro
    let mut v = vec![1, 2, 3];
    v.push(2);
    for x in &v{
        println!("{}",x);
    }

    // Getting element:
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

}
