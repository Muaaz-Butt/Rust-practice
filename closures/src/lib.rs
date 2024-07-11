use std::thread;
use std::time::Duration;
use std::collections::HashMap;

struct Cacher<T>
    where T: Fn(u32) -> u32 
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T> 
    where T: Fn(u32) -> u32 
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
      match self.values.get(&arg){
          Some(&v) => v,
          None => {
              let v = (self.calculation)(arg);
              self.values.insert(arg, v);
              v
          },
      }
    }
}

pub fn generate_workout(intensity: u32, random_number: u32) {
  let mut expensive_result = Cacher::new(|num| {
      println!("Calculating slowly...");
      thread::sleep(Duration::from_secs(2));
      num
  });
  if intensity < 25 {
      println!("Today do {} pushups!", expensive_result.value(intensity));
      println!("Next do {} situps!", expensive_result.value(intensity));
  } else {
      if random_number == 3 {
          println!("Take a break today! Remember to stay hydrated!");
      } else {
          println!("Today run for {} minutes!", expensive_result.value(intensity));
      }
  }
}

mod tests{
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let mut v1 = c.value(1);
        let mut v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}