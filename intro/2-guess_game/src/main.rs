use std::io;
// use rand::prelude::*;
use std::cmp::Ordering;
use rand::Rng;
// Standard library (Prelude)
fn main() {
    // entry point to the program
    println!("Guess my number");
    let secret_number:u32 = rand::thread_rng().gen_range(1..101);
    loop
    {println!("Please input your guess");
    let mut guess = String::new(); // return a new string instance UTF-8 Encoded bit of text

    // ::new indicates the function is associated with String type
    // guess is now empty string an instance of String
    // In Rust variaables are immutable by default.
    io::stdin().read_line(&mut guess).expect("Failed to read input");
    // In Rust, just like variables, references are also immutable

    // If parse is able to parse the string then it returns it as a parameter to Ok enum value, or it throws an Err Enum 
    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    }; // Trim method eliminates carriage returns (\r\n Windows \n Unix like systems)
    println!("You guessed: {}", guess);

    
    // println!("Random number was: {}", secret_number);
    // do not use ; after match case lines do use , instead PRODUCE ERROR
    match guess.cmp(&secret_number) 
    {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {
            println!("You got our secret number"); 
            break;}
    }
    }
}