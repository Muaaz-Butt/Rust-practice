fn main() {
    //Tuples
    
    //Create Tuple::Two ways
    
    let tup1 = (10, 10.1, 'a');
    let tup2: (u8, i32, f64) = (2, 23, 64.11);  // Eplicitly define data types UTF
    
    let (x, y, z) = tup2;         //This method is called destructuring
    
    println!("The value of y: {}", y);
    
    let ten = tup1.0;                       //This way of assigning to variables is called indexing 
    let ten_point_one = tup1.1;
    let a = tup1.2;
    
    println!("The value of a: {}", a);
    
    //Arrays
    
    //In rust, arrays cannot grow or shrink it will remain of the same size and of the same type
    let array = [1,2,3,4];  
    
    //How to access
    let access_array_index_one = array[0];
    
    //If you try to access the index greater than the length it will get panic and give error unlike other languages
    //Other languages access that and give some garbage value but it will give error

}
