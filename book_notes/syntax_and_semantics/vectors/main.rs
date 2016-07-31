// A compilation of notes and lessons from https://doc.rust-lang.org/book/vectors.html

fn main() {
    // A vector is a growable array, like in other languages
    // Note that this is a macro like println! but it uses []. Apparently you can use either in both situations
    // However it's convention to use brackets here and parentheses with println! (and probably other, similar situations)
    // The size of the type must be known at compile time. For everything else there's pointers.
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5];

    // The same syntax exists for filling a repeating value as with arrays
    let v2 = vec![0; 10];

    // Indexing into a vector works the same as an array. However, there is a quirk if you try to use a 
    // variable to index - that variable must be of the 'usize' type

    let foo: usize = 1;
    let bar: i32 = 1;

    // This works
    println!("{}", v1[foo]);

    // This doesn't
    // println!("{}", v1[bar]);

    // Out-of-bounds access fails as you'd expect. If you want to have the vector bounds-check for you then you
    // must use methods like get
    match v1.get(1000) {
        Some(x) => println!("Element 1000 is {}", x),
        None => println!("Array is too short")
    }

    let mut v3 = vec![1, 2, 3];

    // There are three methods of iterating over a vector, depending on references to the values being iterated over
    // and if ownership should be taken of the vector itself. Note that if ownership *is* taken of the vector then it
    // cannot be used again afterwards

    // A reference to each item
    for i in &v3 {
        println!("This is a reference to {}", i);
    }

    // A mutable reference to each item (because the vector itself is mut)
    for i in &mut v3 {
        println!("This is a mutable reference to {}", i);
    }

    // This takes ownership of the vector and its element
    for i in v3 {
        println!("Take ownership of v3 and {}", i);
    }

    println!("This will not work because v3 cannot be used again once we've taken ownership: {}", v3[0]);
}
