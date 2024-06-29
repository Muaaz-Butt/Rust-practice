#[derive(Debug)]

//Defining enum

enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),

}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("{:?}", four);
    println!("{:?}", six);
    println!("{:?}", home);
    println!("{:?}", loopback);
    
    let some_number = Some(5);
    let some_string = Some("Muaaz");
    let absent_number: Option<i32> = None;
    
    println!("{:?}", some_number);
    println!("{:?}", some_string);
    println!("{:?}", absent_number);
}
