use std::env;
use std::process;

use chapter_12_io::run;
use chapter_12_io::Config;

fn main() {
    // First argument is the binary
    // dbg!(&args);

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing the arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for '{}'", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
