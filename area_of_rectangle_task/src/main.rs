#[derive(Debug)]

struct Rectangle {
   width: u32,
   height: u32,
}

fn main() {
    let width1 = 10;
    let height1 = 10;
    
    println!(
    "The area of rectangle {} square pixels. ", area_of_rectangle(width1, height1)
    );
    
    //Refactoring with tuples
    
    let rect1 = (30,50);
    
    println!(
    "The area of rectangle {} square pixels. ", area_of_rectangle_using_tuples(rect1)
    );
    
    //Issue in tuple is it is not readable user will get confused which is for height and which is for width so replacing it with structs
    
    let rectangle = Rectangle {
    	width: 20,
    	height: 10,
    };
    
    println!(
    "The area of rectangle {} square pixels. ", area_of_rectangle_using_structs(&rectangle)
    );
    
    println!("Rectangle is {:#?}", rectangle);
    
}

fn area_of_rectangle(width: u32, height: u32) -> u32 {
    width * height
}

fn area_of_rectangle_using_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_of_rectangle_using_structs(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

/*
if we want to access struct using println!("rect1 is {}", rect1); It will give error so what we have to do is :? add in the curly braces to access and use the trait at the top #[derive(Debug)] example shown in the main function. If we add hash (#) it will display that in more good format
*/






