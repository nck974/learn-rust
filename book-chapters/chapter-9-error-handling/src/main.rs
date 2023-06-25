use std::{error::Error, fs::File, io::ErrorKind};

fn main() -> Result<(), Box<dyn Error>> {
    // Consider scenario not creating the file, therefore not panic on error
    let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    // Shorter version of the same using unwrap or else
    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    // Shorter version:
    // let greeting_file =
    //     File::open("hello.txt").expect("hello.txt should be included in this project");

    // This works only if main returns a result
    let greeting_file = File::open("hello.txt")?;
    Ok(())

    // Crash:
    // panic!("Manual panic");
}
