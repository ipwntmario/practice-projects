const MAX_POINTS: u32 = 100_000; // always immutable - type is required

fn main() {
    // CONSTANTS
    println!("MAX_POINTS is {}", MAX_POINTS);

    // IMMUTABLE VARIABLES
    let x = 5; // immutable variables cannot be changed after set
    println!("x is {}", x); // {} curly brace is a placeholder
    let x = x + 1; // Shadowing
    println!("x is now {}", x);
    let x = x * 2;
    println!("x is now {}", x);

    // MUTABLE VARIABLES
    let mut y = 7; // mutable variable can be changed after set
    println!("y is {}", y);
    y = 8;
    println!("y is now {}", y);
}
