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
1. Matches must cover all branches (otherwise there is a compilation error.
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
