use std::fmt::Display;

/*
The code given below will give error beacuse if the s1 or s2 any if this goes out of scope
and result is still in the scope then it will become dangling reference so what we do is 
make our function and return type generic lifetime so that it will make sure that s1 and s2 
remains in the scope until the result does not go out of scope
*/

/*
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/

// The above function now will be written as

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

//Concept of generics, trait bounds and lifetimes in single function
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
    where T: Display 
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn main() {
    //let r;
    //{
        //let x = 5;
        //r = &x;
    //}
    
    let x = 5;
    let r = &x;
    println!("r: {}", r);
    
    let string1 = String::from("abcd");
    let string2 = "xyzzz";
    
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    
    let novel = String::from("Call me Ismael. Some years ago...	");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    
    let i = ImportantExcerpt{part: first_sentence};
    
    println!("{}", first_sentence);
    
    let string1 = "Hello";
    let string2 = "World!";
    let announcement = "Comparing lengths";

    let result = longest_with_an_announcement(string1, string2, announcement);
    println!("The longest string is: {}", result);
}    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
