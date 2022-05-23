// Set of rules how Rust manages RAM

// In Rust, the garbage collection mechanism is totally different than others. The compiler is responsible for memory management through a system of ownership rules.

// If any rule is violated, then the program won't compile.

// The ownership rules

// Each value in RUST, has a variable that's called its OWNER
// There can only be one OWNER
// When OWNER goes out of scope, the value will be dropped.

// String is unlike other primitive types, is stored in heap.

fn main() {
    let s = String::from ("Hello");
    let mut p  = String::from ("Ukrainian my brother");
    p.push_str(", I support you");
    println!("{}", s);
    println!("{}", p);
    memory_safety();
    clonestr();
    my_main();
}

// In RUST (like most of the languages), the memory requested at runtime
// However, giving the memory back is different from all other languages. When the variable goes out of scope, the memory is freeed with drop keyword, which is implicitly called by RUST.

// ways variables can interact

fn memory_safety() {
    let x = 3;
    let y = x;
    // s1 pointer is on the stack which hold a pointer to the memory that holds the content of string, length of it and a capacity.
    println!("x = {}, y = {}", x, y);
    let s1 = String::from ("Hi mom!");
    let s2 = s1; // not a copy but a reference

    // Now there is a potential double free in the same memory location.

    // println!("{}", s1); // This code won't work, in order Rust to ensure the memory safety
    println!("{}", s2); // this will work though because s1 is invalidated
    
    // Rust will never create "DEEP COPIES" of our data.
}

fn clonestr() {
    let s1 = String::from ("Hi dad!");
    let s2 = s1.clone();

    println!("s1:{} s2:{}", s1, s2);
}

/* OWNERSHIP AND FUNCTIONS */

// Now it is not different from the normal flow with the functions as well. Here is my simple snippet for functions

fn my_main() {
    let s = String::from("Hello mom!");

    let main_str =  use_string(s); // in my_main function s is not valid, its membership goes to my_str argument in use_string function
    let x: i32 = 3;
    use_num(x);
    println!("{}", x); // you can use it here
    println!("{}", main_str); // you can use it here
}// x goes out of scope

fn use_string(my_str: String) -> String {
    println!("{}",my_str);
    // now create a string here and return to main function

    let my_str = String::from("Hello dad!");
    return my_str;
}

fn use_num(y: i32) { // y comes into scope (deep copy of x)
    println!("{}", y); 
} // y goes out of scope