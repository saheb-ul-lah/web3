fn main() {
    


    // Declaring integer variable
    let num: u8 = 5; // u8 means unsigned 8-bit integer, it is optional to specify type, but generally a good practice, it is mandatory in complex datatypes
    println!("The value stored in num is {}", num);

    // Mutability in rust
    num = 10; // This will cause a compile-time error because variables are immutable by default in Rust
    println!("The value stored in num after reassignment is {}", num);

    // To make a variable mutable, use the 'mut' keyword
    let mut mutable_num: u8 = 5;
    println!("The value stored in mutable_num is {}", mutable_num);
    mutable_num = 10; // Now this is valid
    println!("The value stored in mutable_num after reassignment is {}", mutable_num);

    // Declaring floating point variable
    let decimal: f64 = 3.14;
    println!("The value stored in decimal is {}", decimal);

    // Declaring boolean variable
    let is_rust_fun: bool = true;
    println!("Is Rust fun? {}", is_rust_fun);

    // Declaring character variable
    let letter: char = 'R';
    println!("The character stored in letter is {}", letter);

    // Declaring string variable
    let greeting: &str = "Hello, Rust!";
    println!("The greeting message is: {}", greeting);

    // Declaring a tuple
    let tuple: (i32, f64, char) = (42, 3.14, 'R');
    println!("The tuple contains: ({}, {}, {})", tuple.0, tuple.1, tuple.2);

    // Declaring an array
    let array: [i32; 3] = [1, 2, 3];
    println!("The array contains: [{}, {}, {}]", array[0], array[1], array[2]);

    // 
}
