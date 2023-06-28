use std::{
    sync::{
        // mpsc,
        Arc,
        Mutex,
    },
    thread,
    // time::Duration, rc::Rc,
    // time::Duration
};

fn main() {
    // In this scenario the spawned thread does not finish before the main is finished
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1000));
    //     }
    // });

    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1000));
    // }

    // Waiting for all to end
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(100));
    //     }
    // });

    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(100));
    // }
    // handle.join().unwrap();

    // Moving a variable to the thread
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    // Communication between threads
    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let messages = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];
    //     for message in messages {
    //         thread::sleep(Duration::from_millis(1000));
    //         tx.send(message).unwrap();
    //         // println!("message is {}", message); // This wont work as it is already borrowed
    //     }
    // });

    // for received in rx {
    //     println!("Got: {}", received);
    // }

    // Multiple producers
    // let (tx, rx) = mpsc::channel();

    // let tx1 = tx.clone();
    // thread::spawn(move || {
    //     let messages = vec![
    //         String::from("1: hi"),
    //         String::from("1: from"),
    //         String::from("1: the"),
    //         String::from("1: thread"),
    //     ];
    //     for message in messages {
    //         thread::sleep(Duration::from_millis(1000));
    //         tx1.send(message).unwrap();
    //     }
    // });
    // thread::spawn(move || {
    //     let messages = vec![
    //         String::from("2: hi"),
    //         String::from("2: from"),
    //         String::from("2: the"),
    //         String::from("2: thread"),
    //     ];
    //     for message in messages {
    //         thread::sleep(Duration::from_millis(1000));
    //         tx.send(message).unwrap();
    //     }
    // });

    // for received in rx {
    //     println!("Got: {}", received);
    // }

    // Mutex in single thread
    let m = Mutex::new(5);
    {
        // Acquire the lock
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("{:?}", m);

    // Mutex with multiple threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut number = counter.lock().unwrap();
            *number += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Counter: {:?}", counter);
}
