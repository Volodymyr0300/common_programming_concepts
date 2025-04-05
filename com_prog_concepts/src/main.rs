fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6; // This will cause a compile-time error
    // println!("The value of x is: {}", x);

    // const MAX_POINTS: u32 = 100_000;
    // println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // let y = 5;
    // let y = y + 1; // Shadowing
    // let y = y * 2; // Shadowing
    // println!("The value of y is: {}", y);

    // let space = " ";
    // let space = space.len();
    // println!("The value of space is: {}", space);

    // let mut spaces = "   ";
    // spaces = spaces.len();
    // println!("The value of spaces is: {}", spaces);

    // let guess: u32 = "42".parse().expect("Not a number!");
    // println!("The value of guess is: {}", guess);

    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Remainder: {}", remainder);

    let t = true;
    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("Boolean values: {} and {}", t, f);
    println!("Characters: {} and {} and {}", c, z, heart_eyed_cat);
}