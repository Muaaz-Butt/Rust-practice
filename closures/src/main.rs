extern crate closures;

use std::thread;
use std::time::Duration;

use closures::*;


fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() { 
/*  **************************************************************************  */    

// Capturing the environment variables with closures  
    let x = 4;

    let equal_to_x = |z| z == x;
                                  
    let y = 4;

    assert!(equal_to_x(y));
/* 
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    println!("can't use x here: {:?}", x); // Cannot execute this statement because x is moved in closure
    let y = vec![1, 2, 3];
*/
/*  **************************************************************************  */    


    let simulated_intnsity = 26;
    let simulated_random_number = 4;

    generate_workout(simulated_intnsity, simulated_random_number); 
}

