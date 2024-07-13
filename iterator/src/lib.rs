#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

struct Counter {
    count: u32,
}

impl Counter {
  fn new() -> Counter {
    Counter{ count: 0 }
  }
}

impl Iterator for Counter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
      self.count += 1;

      if self.count < 6 {
          Some(self.count)
      } else {
        None
      }
  }
}
mod tests{
    use super::*;

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum () {
        let v1 = vec![1,2,3,4];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();

        assert_eq!(total, 10);
    }

    #[test]
    fn filters_by_size() {
      let _shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
      ];
      let in_my_size = shoes_in_my_size(_shoes, 10);

      let _shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        //Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") }, 
      ];  

      assert_eq!(
          in_my_size, _shoes
      );
    } 

    #[test]
    fn calling_next_directly() {  
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    //Using other iterator methods
    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                     .map(|(a, b)| a * b)
                                     .filter(|x| x % 3 == 0)
                                     .sum();
        
        assert_eq!(18, sum);

    }

}