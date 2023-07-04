// use chapter_17_oop::{Button, Screen, SelectBox};

use chapter_17_oop::Post;

fn main() {
    // Example of using dynamically typed vectors implemeting a trait
    //     let screen = Screen {
    //         components: vec![
    //             Box::new(Button {
    //                 width: 32,
    //                 height: 20,
    //                 label: "Some button".to_string(),
    //             }),
    //             Box::new(SelectBox {
    //                 width: 30,
    //                 height: 40,
    //                 options: vec!["test".to_string(), "test".to_string()],
    //             }),
    //         ],
    //     };

    //     for component in screen.components{
    //         component.draw();
    //     }

    // Test OOP example
    // let mut post = Post::new();

    // post.add_text("I ate a salad for lunch today");
    // assert_eq!("", post.content());

    // post.request_review();
    // assert_eq!("", post.content());

    // post.approve();
    // post.approve();
    // assert_eq!("I ate a salad for lunch today", post.content());

    // println!("All assertions passed!")

    // Same exercise different design pattern
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
    println!("All assertions passed!")
}
