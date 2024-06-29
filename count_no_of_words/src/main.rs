use std::collections::HashMap;

fn main() {
    let mut count_map = HashMap::new();
    
    let text = "Muaaz Butt Zayyad Butt Hammad Butt Muaaz Butt";
    
    //split_whitespace() function extract word from the text 
    for word in text.split_whitespace() {                     
        let count = count_map.entry(word).or_insert(0);                   //Count is referring to that map index so to access we must dereference it
        *count += 1;
    }
    
    println!("{:?}", count_map);
}
