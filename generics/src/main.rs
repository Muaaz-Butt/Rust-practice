
fn find_largest_in_array(array: &[i32]) -> (i32, usize) {
    let mut max_num = array[0];
    let mut max_index = 0;
    
    for (index, &num) in array.iter().enumerate() {
    	if num > max_num {
    	    max_num = num;
    	    max_index = index;
    	}
    }
    (max_num, max_index)
}

fn find_largest_in_array_char (list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest  {
            largest = item;
        }
    }
    largest
}


//Now move to generics and you do not need to make function for each data type

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {               //PartialOrd and copy are traits
    let mut largest = list[0];
    
    for &num in list.iter() {
        if num > largest {
            largest = num
        }
    }
    largest
}

// In Struct Definition

struct Point<T, U> {
    x: T,
    y: U,
}

// In enums 

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

//In method definition

struct Point_<T> {
    x: T,
    y: T,
}

impl<T> Point_<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
 
fn main() {

    let integer = Point{x: 5, y: 10};
    let float = Point{x: 1.3, y: 2.3};
    
    let numbers = [4,12,64,22,1,0,98,2];
    //let (largest_num, index) = find_largest_in_array(&numbers);
    
    let largest_num = largest(&numbers);
    println!("Largest number: {}", largest_num);
    
    let chars = vec!['m', 'n', 'q', 'a', 'r', 's', 'z'];
    //let largest_in_char = find_largest_in_array_char(&chars);
    
    let largest_character = largest(&chars);
    println!("Largest in characters: {}", largest_character);
    
    let p = Point_ {x: 10, y: 2};
    
    println!("p.x() = {}\np.x = {}", p.x(), p.x);
}
  
  
  
  
  
  
  
  
  
  
  
  
    
