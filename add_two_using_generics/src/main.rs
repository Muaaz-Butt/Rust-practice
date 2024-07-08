
#[test]
fn add_two<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    let result = x + y;
    result
}

fn main() {
    let x: i32 = 2;
    let y: i32 = 4;
    let result: i32 = add_two(x, y);
    println!("Result: {}", result);
}