//Defining struct
    
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        //email: email,                          
        //username: username,
        
        //If parameter and the field name are exactly same then we can only write
        
        email,
        username,
        active: true,
        sign_in_count: 1, 
    }
}

fn main() {
    //Creating Instance
    
    let user1 = User {
    	email: String::from("muaazbutt585@gmail.com"),
    	username: String::from("Muaaz Butt"),
    	active: true,
    	sign_in_count: 1,
    };
    
    //If you want to change the value in the struct you need to make the instance mutable
    
    let mut user2 = User {
    	email: String::from("muaazbutt585@gmail.com"),
    	username: String::from("Muaaz Butt"),
    	active: true,
    	sign_in_count: 1,
    };
    
    user2.email = String::from("buttzayyad123@gmail.com");
    
    //Creating a instance that have same values from other instance have two ways
    
    let user3 = User {
    	email: String::from("someone@gmail.com"),
    	username: String::from("someone"),
    	
    	//first way
    	
    	//active: user1.active,
    	//sign_in_count: user1.sign_in_count,
    	
    	//second way
    	
    	..user2
    };
    
    println!("user 2 name is: {}", user2.username);
    
    //Using tuple structs without name field to create different types
    
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    let black = Color(0,0,0);
    let origin = Point(0,2,0);
    
    //Note: These are not fields these are type of fields which can be different and it is not necessary that it should be same
    
    //Access them use dot followed by index
    
    println!("{}", origin.1);
    
}
















