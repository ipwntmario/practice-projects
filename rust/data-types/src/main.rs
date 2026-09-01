fn main() {
    /*
    SCALARS
    */
    let age: u32 = 18; // non-negative 32-bit integer
    let num: i32 = -100; // default for numbers if you don't specify type
    let temperature: f64 = 36.6; // 64-bit floating point number for decimals
    let is_active: bool = true; // true or false
    let grade: char = 'A'; // a single character

    println!(
        "age: {}, num: {}, temp: {}, active: {}, grade: {}",
        age, num, temperature, is_active, grade
    );

    /*
    COMPOUND
    */

    // TUPLE
    let person: (&str, u32, u32) = ("Maya", 29, 50_000);
    println!(
        "{} is {} years old, has the salary {}",
        person.0, person.1, person.2
    );

    // ARRAY
    let scores: [i32; 3] = [90, 85, 100];
    println!("first score: {}", scores[0]);
}
