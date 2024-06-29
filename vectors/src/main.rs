fn main() {
    // Create vector
    let mut v1: Vec<i32> = Vec::new();
    
    //Create and initialize using macro
    let v2 = vec![1,2,3];
    
    //Update vector
    v1.push(1);
    v1.push(2);
    v1.push(4);
    
    //Accessing values
    let third: &i32 = &v1[2];
    let second: Option<&i32> = v1.get(1);
    
    println!("Second: {:?} Third: {}", second, third);
    
    let v3 = vec![1,2,3,4,5,6,7];
    //let does_not_exist_idx = &v3[100];                      //Panic and program will crash.
    let does_not_exist = v3.get(100);                         // Return none, Return type Option<T> Either return value Some() or None.
    
    println!("Does Not Exist: {:?}", does_not_exist);
    
    //let mut v = vec![1, 2, 3, 4, 5];
    //let first = &v[0];                  //Immutable borrow occurs here 
    //v.push(6);                          //Mutable borrow occurs so this code will not run 
    
    //Add 50 to each element in vector
    
    let mut v = vec![0,5,10,15];
    
    for i in &mut v {
        *i += 50;
    }
    
    for i in &mut v {
        println!("{}", *i);
    }
}	
