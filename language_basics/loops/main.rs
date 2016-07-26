// A compilation of notes and lessons from https://doc.rust-lang.org/book/loops.html

fn main() {
    // The simplest type of loop is simply called "loop". It is an infinite loop (but can be broken out of)
    loop {
        println!("going and going and going...");
        break;
    }

    // While loops are fairly standard
    let mut go = true;

    while go {
        go = false;
    }

    // It's fairly important that you do not use "while true" to do an infinite loop - using 'loop' instead allows
    // for compiler optimizations since it knows it will not need to check a conditional every time it loops

    // 'for' loops are not C-style on purpose. Instead they are of the type for *var* in *exp*, where exp is an
    // item that can be turned into an iterator
    for num in 0..10 {
        // this prints 0 through 9, not 10
        println!("{}", num);
    }

    // We can use the enumerate() method to tell how many times we've already looped
    for (num_times_looped, i) in (5..10).enumerate() {
        println!("num_times_looped is {}, i is {}", num_times_looped, i);
    }

    // Note: I originally wanted to do this with an array instead of copying the website example. However, we can't iterate over
    // stock arrays in the way you'd expect. Something to do with the types contained in the array. It'll come up later when
    // discussing iterators
    let lines = "hello\nworld".lines();

    for (linenumber, line) in lines.enumerate() {
        println!("linenumber is {}, line is {}", linenumber, line);
    }

    // 'break' and 'continue' work as you'd imagine they would by default. You can, however, make things more interesting with
    // loop labels. These allow 'break' and 'continue' to apply to any of several named nested loops
    let mut count = 0;
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            // Only count if both are even
            if x % 2 == 1 {
                continue 'outer;
            }
            if y % 2 == 1 {
                continue 'inner;
            } 
            count = count + 1;
        }
    }
}
