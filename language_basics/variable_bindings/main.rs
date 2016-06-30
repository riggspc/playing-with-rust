// A complilation of notes and lessons from https://doc.rust-lang.org/book/variable-bindings.html
// Note: this file will not compile as it is right now - there are lots of warnings about unused variables and such

fn main() {
    // This is the 'basic' way to do it. The 'let' keyword is all you need
    let a = 5;

    // The LHS is a pattern and can be assigned to by the RHS of a similar pattern
    // Patterns are a language concept of their own but can be used to do things like this
    let (b, c) = (2, 3);

    // Rust is statically typed, making 'let' similar to 'auto' in C++ - the language figures it out at compile time
    // However, if we want to we can tell the compiler up front "Hey, this is the type we want this to be", in case
    // there's potential ambiguity (32bit vs 64bit int, for example)
    let d : i64 = 7;

    // By default variables ('bindings' in Rust-speak) are immutable. If you uncomment out the line below the program
    // won't compile
    let e = 7;
    // e = 8;
    // We need to specifically tell Rust that a variable is mutable when we create it with the keyword 'mut'
    let mut f = 8;
    f = 9;

    // Rust bindings must be initialized - there is no 'default' like in C++. As such the following line would not compile
    // let d : i32;
    // println!("{}", d);

    // Scope looks to be very similar to C++ - variables are scoped to the {} block they were created in
    // It does not seem like you can create a variable at the global scope ie. outside of a method

    // Rust allows you to overwrite ("shadow") a variable in the same scope by assigning it another value. This is not
    // the same as mutability - what we're doing here is discarding and overwriting the previous binding with a new one
    // As such we can change things like the type and mutability of a binding
    let g : i32 = 7;
    println!("{}", g); // will print 7
    let mut g = "hello world";
    println!("{}", g); // will print hello world
}