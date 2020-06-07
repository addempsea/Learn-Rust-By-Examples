#![allow(unreachable_code)]

fn main() {
    // if-else
    let n = 15;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");

        // This expression returns an `i32`.
        10 * n
    } else {
        println!(", and is a big number, halve the number");

        // This expression must return an `i32` as well.
        n / 2
        // TODO ^ Try suppressing this expression with a semicolon.
    };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n);

    // loop

    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }

    // labels
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }
    println!("Exited the outer loop");

    // return values from loop

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    // while loops

    // A counter variable
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }

    // for loops

    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz")
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // for in loops

    let mut names = vec!["Bob", "Frank", "Ferris"];
    // borrows names
    for name in names.iter() {
        println!("{}", name)
    }
    // allows edit of names
    for name in names.iter_mut(){
        println!("{:?}", *name = "k")
    }
    // consumes names
    for name in names.into_iter() {
        println!("{}", name)
    }

    // match 
    let number = 7;
    println!("Tell me about {}", number);
    match number {
        1 => println!("very young"),
        7 => println!("prime"),
        _ => println!("very old")
    }

    // match destructing
    let numbers = (3, 7);
    match numbers {
        (x, 6) => println!("first is {}", x),
        _ => println!("very old")
    }

    // match guards
    match numbers {
        (x, y) if x / y > 0 => println!("not ÷"),
        (x, y) if x == y => println!("not ="),
        (x, y) if x < y => println!("not >"),
        _ => println!("nothing")
    }

    // binding
    fn age() -> u32 {
        15
    }
    match age() {
        0 => println!("too young"),
        r @ 1..=10 => println!("young age {}", r),
        r @ 11..=20 => println!("prime {}", r),
        _ => println!("too old")
    }

}
