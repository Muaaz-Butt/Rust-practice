fn main(){
    
    //Mutable and Immutable concept
    
    //let x = 5;      //Immutable we cannot change value of it after we have assigned value;
    
    let mut x  = 5;   // Now we change value of x wherever we want 
    
    println!("The value of x is: {}", x);
    
    x = 6;
    
    println!("The value of x is: {}", x);
    
    //Concept of Shadowing
    //If you have named a variable x then you can again let that variable with the same name 
    //It will shadow the new value in the variable example given below
    
    let y = 5;
    let y = y + 1;
    let y = y * 3;
    
    println!("The value of Y is {}", y);
    
    /*
    
    The other difference between mut and shadowing is that because we’re
    effectively creating a new variable when we use the let keyword again, we
    can change the type of the value but reuse the same name
    
    */
    
    //For example
    
    let spaces = "";
    let spaces = spaces.len();  
    println!("{}", spaces);

    
}
