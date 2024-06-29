enum Coin {
    Penny,
    Nickel, 
    Dime, 
    Quarter,
}

fn value_in_cents (coin: &Coin) -> u32 {
    match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 20,
    Coin:: Quarter => 25,
    }
}


//Patterns that bind to values
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin_State {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents_with_state(coin_state: &Coin_State) -> u32 {
    match coin_state {
        Coin_State::Penny => 1,
        Coin_State::Nickel => 5,
        Coin_State::Dime => 10,
        Coin_State::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(value) => Some(value + 1), 
    }
}

//If we want to check only one condition and ignore all others then we use placeholder(_) as given below
fn match_only_one(x: &Option<i8>) {
    match x {
        Some(3) => println!("Three"),
        _ => println!("Any value that is ignored"), 
    }
} // This is exhaustive and not appropriate when you only want to check one pattern so we have come with the new concept which is if let it will check and compare with only one and return as given below

fn match_only_one_if_let(x: &Option<i8>) {
    if let Some(3) = x {
        println!("Three");
    } else {
        println!("All other values to be ignored");
    }
}

fn main() {
     let penny = Coin::Penny;
    println!("Value of penny: {} cents", value_in_cents(&penny));

    let quarter_with_state = Coin_State::Quarter(UsState::Alabama);
    println!(
        "Value of quarter: {} cents",
        value_in_cents_with_state(&quarter_with_state)
    );
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("{:?}", six);
    println!("{:?}", none);
    
    let x: Option<i8> = Some(3);
    let y: Option<i8> = Some(9);
    
    match_only_one(&x);
    match_only_one(&y);
    
    match_only_one_if_let(&x);
    match_only_one_if_let(&y);
}

























