fn main() {
    another_function(5, 6);
    
    //Concept of expression and statement
    
    let x = 5;
    let y = {
        let x = 3;
        x + 1          /*
                         This statement does not end with semi colon it means it is a expression and return value which is then assigned
                         to y 
                      */
    };
    
    println!("The value of y: {}", y);
    
    //Returning value from function
    
    let num = five();
    println!("The value of num: {}", num);
    
    let new_num = plus_one(x);
    println!("The value of new num: {}", new_num);
}

fn another_function(x: i32, y: i32){
    println!("The value of x: {}", x);
    println!("The value of y: {}", y);
}

fn five() -> i32 {       //i32 is return type of function 
    5
}


fn plus_one(x: i32) -> i32 {
    x + 1
}

/*
Difference between Statements and expressions if I forget learn about it from the book
*/




