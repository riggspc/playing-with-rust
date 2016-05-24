// FizzBuzz

fn main() {
    println!("Welcome to FizzBuzz!");
    println!("If a number is divisible by 3 we print 'fizz'. If it's divisible by 5 we print 'buzz'. Otherwise we just print the number");
    println!("It can be both!\n");

    fizzbuzz();
}

fn fizzbuzz() {
    for i in 0..101 {
        let mod_by_three = i % 3 == 0;
        let mod_by_five = i % 5 == 0;

        if mod_by_three && mod_by_five {
            println!("fizzbuzz!");
        }
        else if mod_by_three {
            println!("fizz!");
        }
        else if mod_by_five {
            println!("buzz!");
        }
        else {
            println!("{}", i);
        }
    }
}