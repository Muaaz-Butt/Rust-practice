fn main() {
    let mut s1 = String::from("Hello");
    let mut s2 = String::from(" World");
    
    s2.push_str(" from Muaaz");
    
    let new_s = s1 + &s2;                    // This will work only for two strings not for many so we will use other method 
    
    let mut tic = String::from("tic");
    let mut tac = String::from("tac");
    let mut toe = String::from("toe");
    
    let mut tic_tac_toe = format!("{}-{}-{}", tic, tac, toe);
    
    println!("{}", new_s);
    println!("{}", tic_tac_toe);
    
    for c in tic_tac_toe.chars() {
        println!("{}", c);
    }
    
}
