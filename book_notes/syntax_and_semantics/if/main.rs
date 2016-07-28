// A compilation of notes and lessons from https://doc.rust-lang.org/book/if.html

// There are some quirks to conditionals that makes this file useful, even if it's not completely necessary

fn main() {
    // The main use of if/else if/else is the same as most other languages. What's interesting is the brace
    // placement in style conventions and lack of necessary parentheses (you can include them, it just results
    // in a compiler warning
    let foo = 1;

    if foo == 1 {
        println!("one");
    } else if foo == 2 {
        println!("two");
    } else {
        println!("something else");
    }

    // If is an expression, meaning it returns a value. We can thus do shorthand not completely dissimilar from ternary
    // operations in other languages like so
    let baz = if foo == 1 {
        10
    } else {
        11
    }; // note the necessary ; here

    // Stylistically it's encouraged to write that instead like this
    let qux = if foo == 1 { 10 } else { 11 };

    // The value returned from an if/else is always the value of the last expression in the branch that was chosen
    // An if without an else always returns ()
}
