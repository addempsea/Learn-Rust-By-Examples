fn main() {
    // We can use this function here, and define it somewhere later
    fizzbuzz_to(100);

    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // Imperative approach
    // Declare accumulator variable
    let mut acc = 0;
    // Iterate: 0, 1, 2, ... to infinity
    for n in 0.. {
        // Square the number
        let n_squared = n * n;

        if n_squared >= upper {
            // Break loop if exceeded the upper limit
            break;
        } else if is_odd(n_squared) {
            // Accumulate value, if it's odd
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    // Functional approach
    let sum_of_squared_odd_numbers: u32 =
        (0..6).map(|n| n * n)                             // All natural numbers squared
             .take_while(|&n_squared| n_squared < upper) // Below upper limit
             .filter(|&n_squared| is_odd(n_squared))     // That are odd
             .fold(0, |acc, n_squared| acc + n_squared); // Sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);

    fn calc_sum_of_multiples(n: u32) -> u32 {
        (0..n).map(|n| n)
              .take_while(|&n| n <= 1000)
              .filter(|&n| is_divisible_by(n, 3))
              .fold(0, |acc, current| acc + current)
    }
    println!("{}" ,calc_sum_of_multiples(9));

    fn calc_sum_of_multiple(n: u32) -> u32 {
        let limit: u32 = n;
        (0..n).map(|n| n)
              .take_while(|&n| n <= limit)
              .filter(|&n| is_divisible_by(n, 3) || is_divisible_by(n, 5))
              .fold(0, |acc, current| acc + current)
    }
    println!("sum of multiples of 3 and 5 up to 1000 is: {}" ,calc_sum_of_multiple(1000))
}

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

// Function that returns a boolean value
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Corner case, early return
    if rhs == 0 {
        return false;
    }

    // This is an expression, the `return` keyword is not necessary here
    lhs % rhs == 0
}

// Functions that "don't" return a value, actually return the unit type `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// When a function returns `()`, the return type can be omitted from the
// signature
fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}
