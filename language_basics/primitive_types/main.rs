// A compilation of notes and lessons from https://doc.rust-lang.org/book/primitive-types.html

fn main() {
    // bools - fairly straightforward. either true or false
    let foo: bool = false;

    // char is also fairly straightforward but has a quirk compared to C++ - it's a Unicode char. This means it's 4 bytes instead of 1
    let bar = "😎";

    // There are lots of different types of numbers. We have signed int (i), unsigned int (u), and floating point (f)
    // These come in different sizes: i/u8, i/u16, i/u32, i/u64, f32, f64
    // The defaults are i32 and f64
    let baz = 2;    // i32
    let qux = 1.0;  // f64

    // There are also two 'special' numeric types whose size depends on the system architecture (32 or 64 bit)
    // these are usize and isize

    // Arrays are, by default, immutable. Just like everything else in Rust :) note that the 'mut' keyword works here
    // [T, N] is the form. T is a templated type, N is a compile time constant
    let a: [i32; 3] = [1, 2, 3];

    // A shorthand exists to fill in all items in an array with a specific value. For example, this fills in all 20 values
    // in 'b' with the value 0
    let b = [0; 20];

    // You can access items the say way as every other language - foo[2]. Arrays are zero-indexed
    // You can also access 'slices' of arrays, which can be accessed like so: this selects [1, 3, 3]
    let c = [0, 1, 2, 3, 4];
    let d = &[1..4];

    // Tuples, like other languages, are ordered lists of fixed size. They can hold different types, like so
    // A single tuple can be called for by using a comma after the value - (0,) is a single value tuple, (0) is a zero
    // in parentheses
    let e: (i64, &str) = (3, "foo");

    // Tuples can be assigned to each other if they contain the same types and arity (number).
    // Fields of a tuple can be accessed through a "destructuring let". This sets f and g to each of the parts
    // of the tuple e
    let (f, g) = e;

    // You can also access parts of a tuple via indexing
    let h = e.0;

    // Functions also have a type and can be used to create function pointers. For information on that see the "Functions"
    // section notes
}