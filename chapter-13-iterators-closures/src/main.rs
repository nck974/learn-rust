// use std::thread;

// #[derive(Debug, PartialEq, Copy, Clone)]
// enum ShirtColor {
//     Red,
//     Blue,
// }

// struct Inventory {
//     shirts: Vec<ShirtColor>,
// }

// impl Inventory {
//     fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
//         user_preference.unwrap_or_else(|| self.most_stocked())
//     }

//     fn most_stocked(&self) -> ShirtColor {
//         let mut num_red = 0;
//         let mut num_blue = 0;

//         for color in &self.shirts {
//             match color {
//                 ShirtColor::Red => num_red += 1,
//                 ShirtColor::Blue => num_blue += 1,
//             }
//         }
//         if num_red > num_blue {
//             ShirtColor::Red
//         } else {
//             ShirtColor::Blue
//         }
//     }
// }

fn main() {
    // EXAMPLE USING CLOSURES
    // let store = Inventory {
    //     shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    // };

    // let user_pref1 = Some(ShirtColor::Red);
    // let giveaway1 = store.giveaway(user_pref1);
    // println!(
    //     "The user with preference {:?} gets {:?}",
    //     user_pref1, giveaway1
    // );

    // let user_pref2 = None;
    // let giveaway2 = store.giveaway(user_pref2);
    // println!(
    //     "The user with preference {:?} gets {:?}",
    //     user_pref2, giveaway2
    // );

    // OWNERSHIP OF CLOSURES
    // let list = vec![1, 2, 3];
    // println!("Before defining closure: {:?}", list);

    // thread::spawn(move || println!("From thread: {:?}", list))
    //     .join()
    //     .unwrap();

    // Iterators
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
    let sum: i32 = v1_iter.sum();
    println!("Sum is {}", sum);
    // let sum_2:i32 = v1_iter.sum(); // can not be done already moved because iterator is consumed
    let new_iter: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    dbg!(new_iter);
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
