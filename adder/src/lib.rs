#[derive(Debug)]
pub struct Rectangle {
  length: u32,
  width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two_(a: i32) -> i32 {
    a + 2
}

pub fn greetings(name: &str) -> String {
    //String::from("Hello")
    format!("Hello {}!", name)
}

fn prints_and_returns_10(a: i32) -> i32 {
  println!("I got the value {}", a);
  10
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
  }
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    #[ignore]
    fn larger_can_hold_smaller() {
        let larger = Rectangle{ length: 8, width: 7};
        let smaller = Rectangle { length: 5, width: 1};

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    #[ignore]
    fn smaller_cannot_hold_larger() {
      let larger = Rectangle{ length: 8, width: 7};
      let smaller = Rectangle { length: 5, width: 1};

      assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[ignore]
    fn it_adds_two_() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[ignore]
    fn greetings_contains_name() {
        let result = greetings("Muaaz");
        assert!(
            result.contains("Muaaz"),
            "Greetings did not contain name, value was `{}`", result
          );
    }

    #[test]
    #[ignore]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }
    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

