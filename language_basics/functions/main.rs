// A complilation of notes and lessons from https://doc.rust-lang.org/book/functions.html

fn main() {
    // print_number(5);
    print_number(add_two_numbers(5, 5));
    print_number(semicolon_test());
}

// Similar to specifically typing variables (bindings) - specify the type of parameters like so
// Note that you *must* specify types in the function declaration. You can't just hope the compiler
// figures it out (like 'let')
fn print_number(num : i32) {
    println!("Number is {}", num);
}

// If you don't specify a return type it just defaults to something similar to "void" in C++
// You can specify a return like so
// Side note: I'm not a fan of the style of having the brace on the same line as the declaration,
// especially with a return type there too since it's harder to see the return type at a glance
// when skimming code. However, it appears that's the suggested way of doing it (both in the example
// and the Rust syntax in my editor)
fn add_two_numbers(num1 : i32, num2 : i32) -> i32 {
    // This is an interesting thing: the last line of a method determines what it'll return
    // And yes, not having a semicolon is intentional - if we had one we'd get a compilation error
    // saying that not all control paths return a value
    // This is because "num1 + num2" is an expression while "num1 + num2;" is a statement. An expression
    // returns a value inherantly while a statement does not
    num1 + num2
}

// Statements vs. Expressions is fundamental in Rust. Like it's said above, the difference between expressions
// and statements is that expressions return a value while statements do not
// Using 'let' to assign a value to a variable is *not* an expression and thus doesn't return a value, which
// might be useful in chaining in other languages (eg. x = y = z = 5). This is what's called a 'declaration statement'.
// Note that assigning to an already bound variable is an expression but the return result is an empty tuple.
// So chaining is not useful even then.
// However, like seen above you can not include a semicolon to have a statement become an expression and return a value
fn semicolon_test() -> i32 {
    // The below returns 34 inherantly. But it must be the last thing in the method. I suppose it's a type of shorthand...
    // 34

    // This returns in a more traditional way
    // return 30;

    // This...also returns 24, I guess. Rust is weird.
    return 24

    // A note on the above: using the 'return' keyword is used for early returns. It can be used 'traditionally', like
    // in C++ or Java or similar, as the last return of a method but that's considered bad style. The Rust way is to
    // use the 'return' keyword only for early returns
}

// Diverging functions are functions in Rust that do not return. Here's an example:
// You can set RUST_BACKTRACE environment variable to 1 to see the stack trace at the crash site
fn diverges() -> ! {
    // Forces a crash
    panic!("this method never returns");
}

// We can set a variable binding to a function pointer
fn fn_pointer() {
    // Type inference can be used here - here's an example of not using it though
    let pointer1: fn(i32, i32) -> i32 = add_two_numbers;
    pointer1(32, 54);

    // Type inference for a function pointer
    let pointer2 = add_two_numbers;
    pointer2(21, 21);
}