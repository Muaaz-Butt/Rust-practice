use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    
    //Insert value(String) with key pair(i32)
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);
    scores.insert(String::from("Yellow"), 5);	    
    //Access 
    
    let team_name = String::from("Blue");
    
    let score = scores.get(&team_name);            //Return Option<&T>

    println!("{:?}", score);
    
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
//.entry() this function will check if the value already exist in the hashmap or not if it exists it will no insert the value for inserting we   need one more function which is .or_insert() to insert the value. to overwrite the previous value with same key value pair just add it without using any entry or insert function. Example given below

    scores.entry(String::from("Black")).or_insert(120);
    scores.entry(String::from("Yellow")).or_insert(10);   // This will not add in the hash map
    
    println!("{:?}", scores);     
}
