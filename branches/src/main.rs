use std::io;	

fn main() {
    // If conditions 
    let number = 3;
    if number < 5 {
        //println!("Condition was true");
    } else {
        //println!("Condition was false");
    }
    
    //Program to check whether the number is +ve, -ve or zero
    let mut input = String::new();
     
    println!("Enter a number: ");
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let num: i32 = input.trim().parse().expect("Please type a valid number");
    
    if num > 0 {
        println!("Number is positive");
    } else if num < 0 {
        println!("Number is negative");
    } else {
    	println!("Number is zero");
    }
    
}
