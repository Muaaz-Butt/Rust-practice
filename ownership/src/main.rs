fn main() {
    let mut s1 = String::from("hello");
    s1.push_str(", world!");
    println!("{}", s1);
    let s2 = s1;
    println!("{}", s2);
    
    //Testing of length and capacity in strings
    let mut s = String::from("hello");
    println!("Initial length: {}", s.len());
    println!("Initial capacity: {}", s.capacity());

    // Keep adding strings
    s.push_str(", world!");
    println!("After first push: length: {}, capacity: {}", s.len(), s.capacity());

    s.push_str(" This is a test.");
    println!("After second push: length: {}, capacity: {}", s.len(), s.capacity());

    s.push_str(" Adding more data to see reallocation.");
    println!("After third push: length: {}, capacity: {}", s.len(), s.capacity());
    
    s.push_str(" Adding more data to see reallocation.");
    println!("After fourth push: length: {}, capacity: {}", s.len(), s.capacity());
    
    s.push_str(" Adding more data to see reallocation.");
    println!("After fifth push: length: {}, capacity: {}", s.len(), s.capacity());
}





/*
Rules for ownership: 
1. Each value in rust has a variable that is called it's owner
2. There can be only one owner at a time
3. When the owner goes out of scope the value is dropped

String and String literals:
String literals are immutable and it is hardcoded into our program but sometime we need to take inout from the user for that 
Rust has other type which is String that is mutable and it is allocated on heap

Difference between String and String literals:
The main difference is how they deal with the memory

Memory and Allocation:
The memory must be requested from the operating system. 
It should be returned to the operating system when we have used it. 

The first part is done by us when we call Stirng::from
but the second part is done by the operating system when the variable goes out of scope it automatically calls the drop function at the closing bracket and return the memory to the operating system.

In other languages there is a concept of Garbage Collector (GC) which keeps the track of the memory and free it and it is user 
responsibilty.

In C++ deallocating memory is called Resource Acquisition Is Initialization (RAII).

//Double free error:
let s1 = String::from("hello");
let s2 = s1;
When we try to run this code this will copy the pointer and that is pointing to the same string.
So when s1 and s2 goes out of scope both will try to free the same memory and this is called double free error. 

In rust they have not used the concept of deep copy and shallow copy instead of they have used the concept of MOVE what move acutally do when we do let s2 = s1. It will invalidate the s1 the s1 is moved to s2 now if we try to access s1 it will no longer exist. But if we want both of them to exist then we use .clone() function to copy the data to s2 and now s2 points to the new new allocated string in heap and s1 points to its own allocated string on heap.


*/
