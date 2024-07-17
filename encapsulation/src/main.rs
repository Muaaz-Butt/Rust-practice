pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
      self.list.push(value);
      self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop() ;
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },

            None => None,
        }
    }

    pub fn average(&mut self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64
    } 
}

fn main () {
  let mut collection = AveragedCollection {
    list: Vec::new(),
    average: 0.0,
};

println!("Initial average: {}", collection.average());

collection.add(10);
println!("Average after adding 10: {}", collection.average());

collection.add(20);
println!("Average after adding 20: {}", collection.average());

collection.add(30);
println!("Average after adding 30: {}", collection.average());

collection.remove();
println!("Average after removing last element: {}", collection.average());

collection.remove();
println!("Average after removing last element: {}", collection.average());

collection.remove();
println!("Average after removing last element: {}", collection.average());

if collection.remove().is_none() {
    println!("Tried to remove from empty collection. Average is still: {}", collection.average());
}
}