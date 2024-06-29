fn main() {
    //While loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");
    
    //for loops using iterator
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
    println!("the value is: {}", element);
    }
    
    //for loops using range
    for number in (1..4).rev() {          //.rev used to reverse range or iterators etc
    	println!("{}!", number);
    }
    println!("LIFTOFF!!!");

}
