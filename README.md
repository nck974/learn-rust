# Learn rust

This repository contains my steps learning rust.

## Day 1

1. Complete: <https://doc.rust-lang.org/book/ch01-00-getting-started.html>
1. Complete: <https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html>

Summary:

1. Learning the basic of using cargo.
1. Basic syntax of rust.
1. How to use expect and Ok/Err;

## Day 2

1. Complete: <https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html>
1. Complete: <https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html>
1. Complete <https://doc.rust-lang.org/book/ch05-00-structs.html>

Summary:

1. Chapter 3:
1. Learn about mutable variables (default is not mutable)
1. Learn basic types (signed unsigned of float an integer, bool, char, tuple, array, vector ).
1. Char is unicode.
1. Access tuples with dot notation.
1. Arrays are fixed size.
1. Rust does not allow memory access to array length + 1, it panics (exits).
1. Function definition location does not matter.
1. Expected type is always mandatory in input of functions.
1. Statement -> return value. Expression -> evaluate to result a value.
1. Expressions do not have a trailing semicolon (no `return` syntax is needed but possible if it does not return at the end, then ; is needed).
1. Returning type must be written explicitly in functions.
1. Comments are done with `//`.
1. Ifs do not require parenthesis.
1. If does not accept other than bool (e.g. positive number throws error).
1. Exit a loop with break (a value can be returned together with the break).
1. Loops can be labeled `'counting_up: loop {`. Note the single quote.
1. Chapter 4:
1. Stack and heap are parts of the memory.
1. Stack -> The stack stores values in the order it gets them and removes the values in the opposite order (each value must have fixed size) (Fast).
1. Heap -> Less organized (empty spot in memory is found) (slower, requires allocation). Ownership aims to manage heap data.
1. Ownership rules:
1. Each value has an owner.
1. Only one owner at a time.
1. When owner is out of drop the value is dropped.
1. Literal string are immutable, therefore exists Strings to make mutable strings.
1. There are no shallow copies, instead they are `move`.
1. To make a deep copy (twice in heap, and therefore slow) use `clone`.
1. Sending a parameter to a function also produces a move on the parameter.
1. Rust takes care of freeing the memory when a value gets out of scope, important to take care of that when sending the value to a function which is no returning something back to that variable. Instead may be preferable to send a reference with `&variable`.
1. To de-reference use operator `*`.
1. If something has to be mutable in a function from the reference, the passing the reference mutable is indicated with `variable: &mut String`.
1. It is only possible to borrow mutable reference once.
1. Dangling references are not possible.
1. Reference rules:
1. At any given time, you can have either one mutable reference or any number of immutable references.
1. References must always be valid.

1. Chapter 5
1. Structs look like classes/models.
1. individual fields can not be made mutable, all or nothing.
1. There is a shortcut of writing structs that have the same parameter and field name, just write it once.
1. There is also a syntax `..` to send one object to other all parameters unless something else is explicitly defined.
1. There are named tuples like Point(0,1,2) which do not have named parameters.
1. You can print debug prints with `dbg!`.
1. Methods are not implemented directly in the struct but extended with `impl`.
1. Methods always have self parameter as reference, and if the object is modified they are  mutable.
1. All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl.

Exercises:

1. Print christmas carol

## Day 3

1. Complete: <https://doc.rust-lang.org/book/ch06-00-enums.html>

Summary:

1. Chapter 6:
1. Enums can take parameters and work as a function.
1. Enums can take many types.
1. Null does not exist in rust but `Option` with  `None` can be used for that purpose.
1. Match can be used as a case that not only compares booleans but also types in the condition.
1. Matches can be used to return values of parametrized enums, taking the parameter on the enum and doing some action.
1. Matches must cover all branches (otherwise there is a compilation error).
1. A last condition with a parameter name can be used to catch all exceptions.
1. To avoid putting that last condition in some scenarios you can use the syntax `if let`.

## Day 4

1. Complete: <https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html>
1. Complete <https://doc.rust-lang.org/book/ch08-00-common-collections.html>

Summary:

1. Chapter 7:
1. A crate is the smallest unit considered by the compiler.
1. Two types of crate:
1. Binary: Programs done until now.
1. Library: (Used by other rust programs.)
1. Package: Bundle of one or more crates. -> defined in `cargo.toml`.
1. Each package has a root crate `main.rs` or `lib.rs`.
1. Imports of other modules can be done from the root crate with `mod <crate_name>`.
1. Other submodules must be stored in a folder with the name of the module.
1. Modules must be explicitly made public with `pub mod`.
1. `use` can be used to avoid repetition.
1. Two types of paths:
1. Absolute (from `crate`).
1. Relative: `self` or `super`.
1. Path separator is `::`.
1. Api guidelines: <https://rust-lang.github.io/api-guidelines/>
1. Struct parameters must be explicitly made public.
1. In an import with `use` an alias can be be provided with `as`.
1. Adding external packages can be done through the `cargo.toml` package.
1. Multi-import can be done with `use std::{cmp::Ordering, io};` or even with self: `use std::io::{self, Write};`.
1. Also a wildcard can be used to import `use std::collections::*`
1. Chapter 8:
1. Three types of collections:
1. Vector: variable number of elements (only of one type).
1. String: Collection of characters.
1. Hashmap: collection with keys (dict).
1. Syntax to create a vector `vec![1, 2, 3];`.
1. To modify a vector in a loop it has to be dereferenced.
1. There are two strings:
    1. String slices `str` or usually `&str`.
    1. String as a MUTABLE type `String` (Comes from std library rather than the core).
1. Both are UTF8 encoded.
1. Iterating over strings is not done by looping through the string as there are issues with characters that take more bytes, therefore rust provides methods in the String to access bytes, chars...
1. There is a type `HashMap` in the std library (collections), that works as a dictionary in python.
1. Values entered in a hashmap that implement `copy` will be copied, others like String will be moved and be left invalid.

## Day 5

Buy: Ultimate Rust Crash Course

1. Complete the following sections:
    1. Introduction.
    1. Fundamentals.
    1. Primitive types and Control Flow.
    1. The heart of rust.
1. Implement two pointer pattern.

## Day 6

1. Complete udemy exercises:
    1. Exercise A.
    1. Exercise B.
    1. Exercise C.

## Day 7

1. Complete rust book exercises of chapter 8.
1. Complete Udemy exercises:
    1. Exercise D.
    1. Exercise E.

## Day 8

1. Chapter 9:
    1. Two types of errors:
        1. Recoverable: A `Result<T, E>` is returned.
        1. Not recoverable: The program panics (exit).
    1. On panic per default rust cleans the memory. But it can be left to the OS configuring it:

        ```toml
        [profile.release]

        panic = 'abort'
        ````

    1. The macro for crashing is `panic!`.
    1. $Env:RUST_BACKTRACE=1 can be set to show the whole stacktrace on panic.
    1. The operator `?` can be used as as a shortcut to propagate errors from a function. This removes boilerplate. Only works with functions that return a `Result` or `Option`.

## Day 9

1. Complete the following Udemy sections:
    1. The meat of Rust.
1. Complete Udemy exercises:
    1. Exercise F.
    1. Exercise G.
1. Chapter 10:
    1. Generics in a function are indicated just before the parameters: `fn function<T>(list: &[T]) -> &T` and `T` is usually used.
    1. Generics can be used to provide dynamic types to the arguments of an enum.
    1. Generics do not lower the performance.
    1. Traits are similar to interfaces.
    1. Traits can be provided as parameters: `pub fn notify(item: &impl Summary) {...`.
    1. Multiple traits can be added: `pub fn notify(item: &(impl Summary + Display)) {`.
    1. Where clause can be used to specify the traits of the generics:

        ```rust
        fn some_function<T, U>(t: &T, u: &U) -> i32

        where
            T: Display + Clone,
            U: Clone + Debug,
        {
            ...
        }
        ```

    1. Lifetime annotation prevent dangling references.
    1. Lifetimes annotations may be needed to know when somethings goes out of scope. Syntax: `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`.
    1. Lifetime annotations can also be added tu structs `struct ImportantExcerpt<'a> {`.
    1. Lifetime annotations may not be needed thanks to elision rules done by Rust.
    1. There is a special lifetime `'static` that makes the reference live through the entire program.

1. Implement traversal matrix pattern.

## Day 10

1. Complete Udemy exercises:
    1. Exercise H.
    1. Invaders:
        1. Add dependencies.
        1. Setup audio.
        1. Setup rendering and multithread.

## Day 11

1. Complete Udemy exercises:
    1. Invaders:
        1. Create player.
        1. Create shots and make player shoot.
        1. Create invaders.
        1. Finish game with win/lose.
1. Finish Udemy course Ultimate Rust Crash Course.

## Day 12

1. Chapter 11
    1. A test is a function annotated with `#[test]`.
    1. There are some built in macros for comparing like `assert!`.
    1. It is possible to just return a `Result` instead of doing an assertion.
    1. Tests can be multithreaded out of the box `cargo test -- --test-threads=1`.
    1. Output can be displayed with `cargo test -- --show-output`.
    1. The annotation `#[cfg(test)]` does not include the tests in the compiled artifact.
    1. Integration tests are usually placed in a `test` folder.
    1. Functions from `main.rs` can not be tested. The functionality should live in `lib.rs`.
1. Start web API project `diogenes-rs`.

## Day 13

1. Chapter 12 (12.1, 12.2, 12.3):
    1. Two dashes indicate that the commands passed are for the program and not for cargo: `cargo run -- <program args>`.
    1. The first argument is the binary.
    1. `std::fs` can be used to read files.
    1. Using `.clone()` lowers the efficiency of the program. Is not ideal to use it to fix ownership problems.
    1. `::new()` methods are expected to not fail, therefore has more meaning using something like `::build()` if it could crash.

## Day 14

1. Chapter 12 (12.4, 12.5, 12.6):
    1. use `std::env` to get environment variables.
    1. The macro `eprintln!` can be used to write to `stderr`.
1. Chapter 13:
    1. Rust includes many functional programming features.
    1. Closures in rust are anonymous functions.
    1. The syntax for a closure is `|<args>| {<code>}`.
    1. The type of the closures can be many times inferred and does not need to be explicitly written. But it can be done with `|a: i32| -> i32 {...}`.
    1. Single line closures do not need brackets.
    1. `move` can be used to force the closure to take ownership. This prevents the variable outliving its current function.
    1. Closures can take different traits `Fn` (don't move, don't mutate),  `FnOnce` (move), `FnMut` (Don't move but mutate).
    1. Iterators in Rust are lazy loaded.
    1. An iterator implements the trait `Iterator`. The trait contains an associated type.
    1. An iterator has to be mutable as it changes each time `next()` is called. If the references are also mutable then call `iter_mut()` instead of just `iter()`.
    1. Some iterator methods may consume the iterator making it unusable after it is called.
    1. Other methods yield a new iterator (like `map`).
    1. Iterators are not slower than normal loops.

## Day 15

1. Chapter 14:
    1. There are two main profiles `dev` and `release` which can be configured in the `Cargo.toml`.
    1. `opt-level` is the number of optimizations rust applies at compile time.
    1. Documentation comments are done with `///`. Markdown can be used in them.
    1. `cargo doc --open` Can be used to generate a document with the documentation.
    1. The code in markdown blocks will be tested by `cargo test`.
    1. Item (like the whole module) comments can be done with `//!`.
    1. `pub use self::<path>;` can be used to change the structure that is exposed to an api user of the library.
    1. If the library wants to be published the metadata can be written in `Cargo.toml`.
    1. Yanking a version disables it for future use.
    1. A `workspace` can be used to split the code in smaller modules. Each of those modules will have to be published as a separated crate.
    1. Cargo can be extended with new commands by just adding them to the path, for example cargo-new-command can be executed as `cargo new-command`.
        1. They will be automatically added to the `cargo --help` menu.

## Day 16

1. Chapter 15 (15.1, 15.2, 15.3, 15.4):
    1. `&<type>` is a reference to a point in memory or pointer.
    1. Smart pointers are not just the reference but also contain metadata.
    1. Smart pointers allow to have more than one owner by keeping track of the umber of owners.
    1. Smart pointer can not just borrow the data but own it.
    1. `String` or  `Vec<T>` is a type of smart pointer.
    1. Smart pointers are usually implemented with a `struct` that implement the traits:
        1. `Deref`: Allow to behave like a reference.
        1. `Drop`: Customize what happens when it goes out of scope.
    1. A `Box<T>` allows to store memory in the heap instead of the stack where only a pointer is stored without performance overhead more than the storing.
    1. A `Box` is useful for recursive objects where the compiler does not know how to get the size of the object.
    1. Custom dereference can be implemented adding the trait `Deref` or `DerefMut` to a struct.
    1. The `Drop` trait lets you implement what happens when something goes out of scope.
    1. `Rc<T>` is a reference counter, when it does not contain any reference it can be cleaned up (only for single threaded).

1. Implement sliding window pattern.

## Day 17

1. Chapter 15 (15.5, 15.6):
    1. `RefCell<T>` can be used mutate immutable objects by telling the compiler not to enforce the rules for us.
    1. Using `RefCell<T>` ownership/borrowing rules are enforce at runtime instead of compile time.
    1. `RefCell<T>` only works in single threaded scenarios.
    1. The value inside `RefCell<T>` can be mutated even if `RefCell` is immutable.
    1. `RefCell<T>` can be combined with `Rc<T>` to have multiple owners with immutable references that can be mutated.
    1. `Mutex<T>` is the thread safe alternative to `RefCell<T>`.
1. Chapter 16 (16.1):
    1. Rust is implemented with concurrency in mind.
    1. A thread can be created with `thread::spawn`.
    1. On the returned object `.join()` can be called to wait for the thread to finish.

## Day 18

1. Chapter 16 (16.2, 16.3, 16.4):
    1. Threads can communicate with other threads with `channels`.
    1. A channel is dropped if the receiver or the sender closes it.
    1. `recv` blocks the thread, `try_recv` does not and returns a result.
    1. In `std::sync::mpsc` multiple transmitters can be clone for one receiver.
    1. `std::sync::Mutex` can be used to access data once per thread at a time.
    1. `Arc` is an atomic counter that will work in multithread scenarios, like `Rc` in single thread.
    1. `Mutex` is not free of dead locks (threads waiting to each other for ever).
    1. In order for something to be thread safe it has to have the traits `Send` and `Sync`, or be composed of elements that have them.

1. Chapter 17 (17.1)
    1. Rust has some OOP characteristics like:
        1. Structures with data an methods associated `trait` + `struct`.
        1. Public `pub` methods that are private per default, therefore encapsulation for public APIs.
        1. It does not have inheritance per se as it may not be considered an ideal programming pattern for rust.

## Day 19

1. Chapter 17 (17.2, 17.3)
    1. Dynamic typed vectors are possible thanks to creating a `Vec<dyn Trait>`, where `Trait` is the Trait that each element has de implement.
    1. Some problems can be solved in rust by designing the problems to be caught by the compiler instead of using a traditional OOP solution.

## Day 20

1. Chapter 18:
    1. A match consists of all possibilities, a wild card to match everything else is `_`. For example `_ => Some(1)`.
    1. `if let` and `else if let` can be combined with different variables for more complexity that just a `match`. But not all branches are checked by the compiler.
    1. Functions, variables, match/if/while, all can take a pattern.
    1. Patterns can be:
        1. Refutable: Patterns that can fail. (Like `if let x = Some(5)`, because it can be None).
        1. Irrefutable: Match all possibilities (like `let x = 5`).
    1. Multiple patterns can be used in a single place with an `OR` where the syntax is `|` instead of the usual `||`.
    1. The same can be done with ranges `1..2`.
    1. Warnings on unused variables can be ignored with a `_` prefixing the variable name.
    1. Parts for a match can be ignored with `..` Example: `Point { x, .. } => println!("x is {}", x),`.
    1. A match can contain an extra condition called match guard, as `Some(x) if x % 2 == 0`. The compiler will not look for exhaustiveness.
    1. `@` operator allows to add an extra check within a parameter: `id: id_variable @ 3..=7`

## Day 21

1. Chapter 19 (19.1):
    1. Some memory safety ensured during runtime may limit some programs. Therefore rust allows you to write some unsafe code at your own risk.
    1. To make a code unsafe wrap it with `unsafe`.
    1. Is best practice to wrap the unsafe code with a public safe API.
    1. The following is allowed in unsafe code:
        1. Dereference raw pointers:
            1. There is a new type row pointer: `*const` or `*mut`.
            1. Can dereference that row pointer only in an unsafe block.
        1. Calling an unsafe function.
        1. Using extern functions: Is used to interact with other languages.
        1. Modify a mutable static variable.
        1. Implement unsafe traits.
        1. Use unions (like structs used to interface with other languages).

## Day 22

1. Chapter 19 (19.2)
    1. A `trait` can take a dynamic `type` that must not be known until the trait is implemented. Instead of generics this is just implemented with one type instead of multiple.
    1. A default type can be given to the trait.
    1. A collision of names between trait implementation and struct implementation can be solved calling `<trait>::<method>(&<struct_instance);`.
    1. It is possible to have a trait dependency. Where trait A requires trait B, then struct C implements B instead of A.
    1. Some new types can be defined as wrappers to be used within a trait.

## Day 23

1. Chapter 19 (19.3, 19.4)
    1. New types can be used to create aliases of other types. For example: `type Kilometers = i32;`.
    1. Also they can be used to reduce boilerplate in more complex types.
    1. There is a "no type" `!` which indicates that a function will never return.
    1. Functions can be passed as parameters with this syntax: `f: fn(i32) -> i32`.
    1. It is also possible to return them as long as rust knows how much size it takes.

## Day 24

1. Chapter 19 (19.5)
    1. Macros are code that write other code.
    1. Macros are more complex than functions.
    1. Declarative Macros:
        1. Write something similar to `match`.
    1. Procedural Macros:
        1. Receive some code, operate on it, and return some other code.
    1. Function like Macros:
        1. Operate like a function but with more features, like using an unknown number of parameters.

## Day 25

1. Chapter 20:
    1. Build a single threaded http server.

## Day 26

1. Chapter 20:
    1. Make a single thread http server multithread.
1. Finish rust book.

## Day 27

1. Start project [`gpx-geo-filter`](https://github.com/nck974/gpx-geo-filter).
    1. Read directory.
    1. Extract first point in file.
    1. Prefilter based on the distance.

## Day 28

1. Project `gpx-geo-filter`:
    1. Refactor code into smaller modules.
    1. Improve distance calculation from distance between coordinates to distance in km.
    1. Write basic unit tests.
    1. Create multithread filtering function. Speed reduced from 10sec to 2 sec for 200 files.
    1. Change multithreading to use `rayon`.
    1. Optimize regex by compiling it just one time.
    1. On file reading skip lines that do not start with the expected trackpoint tag.

## Day 29

1. Project `gpx-geo-filter`:
    1. Switch regex of complete file by XML parsing using `quick_xml`.
    1. Bug found with different gpx formatting.

## Day 30

1. Project `gpx-geo-filter`:
    1. Fix latitude and longitude order.
    1. Read complete file using quick_xml.
    1. Complete reading for 6000 files is around 40 seconds.
    1. Split lists of prefiltering to skip checking files that already have a point in the area.
    1. Copy the files to a new folder.
    1. Total time with the latest changes including copying is around 4 seconds for 6000 files with 4 threads.

## Day 31

1. Project `gpx-geo-filter`:
    1. Refactor code to only expose a single public library.
    1. Refactor code to smaller more meaningful functions.

## Day 32

1. Project `gpx-geo-filter`:
    1. Expose as a command line tool the needed parameters.
