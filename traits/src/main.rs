use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self{
            x,
            y,
        }
    }
} 

impl <T: PartialOrd + Display> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())                             //Default implementation
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    /*fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }*/
    
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    /*fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }*/
    
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

//pub fn notify(item: &impl Summary) {
    //println!("Breaking news {}", item.summarize());
//}

pub struct Test {
    name: String,
    id: i32,
}

impl Summary for Test {
    fn summarize_author(&self) -> String {
        format!("@{}", self.name)
    }
}

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news {}", item.summarize());
}

//trait bound using where clause it is helpful when we are using many tratis

/*fn some_function<T, U>(t: &T, u: &U) -> String
  where 
      T: Display + clone,
      U: Debug + Clone
{      

}*/


//Find largest without using copy trait



fn main() {
    let tweet = Tweet {
        username: String::from("Muaaz Butt"),
        content: String::from("You can and you will"),
        reply: false,
        retweet: false,
    };
    
    println!("1 new tweet: {}", tweet.summarize());
    
    let test = Test {
        name: String::from("muaaz"),
        id: 1,
    };
    
    notify(&test);
    notify(&tweet);
    
    let pair = Pair{
        x: 10,
        y: 1,
    };
    
    pair.cmp_display();
}
