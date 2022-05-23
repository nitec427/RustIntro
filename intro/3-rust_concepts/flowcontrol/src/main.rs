fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition true");
    } else {
        println!("Condition false");
    }

    // if condition must always be boolean unlike other languages. Integer values are not allowed

    // you can use let - if statement like in any other language

    let value = if number == 1 {1} else {number};
    println!("Number was {} ", value)
}
