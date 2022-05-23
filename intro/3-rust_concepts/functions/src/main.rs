fn main() {
    println!("Hello, world!");
    another_world("C'mon");
    print_number(34);
    cur_weather(22, 'C');
    statem_expression();
    println!( "{}", func_return_val(3));
}
fn another_world(boly:&str) {
    println!("Another world! {}", boly);
}
fn print_number(number:i32){
    // int32
    println!("Here is your number {}", number);
}

fn cur_weather(degree: u8, units: char){
    println!("The weather is now {}°{} ", degree, units);
}

fn statem_expression(){
    // The following is valid and an expression in Rust

    let y = {
        let x = 4;
        x + 1 // do not end with semicolon because it is an expression
    };
    println!(" {} ", y);
}

fn func_return_val(x: i32) -> i32 {
    x * (-4)
}
// fn func_return_val(x: i32) -> i32 {
//     x * (-4); // functions absolutely do not evaluate to any value so either use expression or use return keyword
// }