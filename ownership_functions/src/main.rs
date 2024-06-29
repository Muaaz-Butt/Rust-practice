fn main() {
    let s = String::from("hello");
    // s comes into scope
    takes_ownership(s);// s's value moves into the function...
    // ... and so is no longer valid here
    
    //println!("{}", s);  because of ownership rule s went into the function and will not be accessed after wards
    
    let x = 5;// x comes into scope
    makes_copy(x);// x would move into the function,
    
    
    // but i32 is Copy, so it's okay to
    // still use x afterward
    
    
} // Here, x goes out of scope, then s. But because s's value was moved,
    // nothing special happens.


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} 


// Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.


fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

/*

We can return multiple values from the function in the form of tuple as given below

fn main() {
let s1 = String::from("hello");
let (s2, len) = calculate_length(s1);
println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
let length = s.len(); // len() returns the length of a String
(s, length)
}

*/

