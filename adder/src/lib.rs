pub fn add(left: i64, right: i64) -> i64 {
    println!("Adding {left} + {right}");
    left + right
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

#[allow(dead_code)]
impl Rectangle {
    fn can_contain(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_positive_numbers() {
        let result = super::add(420, 69);
        assert_eq!(result, 489);
    }

    #[test]
    fn adds_two_negative_numbers() {
        let result = super::add(-420, -69);
        assert_eq!(result, -489);
    }

    #[test]
    fn adds_positive_to_negative_numbers() {
        let result = super::add(420, -20);
        assert_eq!(result, 400);
    }

    #[test]
    #[should_panic]
    fn adds_that_result_in_overflow_exits_with_panic() {
        let _ = super::add(9223372036854775807, 1);
    }
  
    #[test]
    fn bigger_rectangle_can_contain_smaller() {
        let bigger_rectangle = Rectangle {
            width: 20,
            height: 20
        };

        let smaller_rectangle = Rectangle {
            width: 19,
            height: 19
        };

        assert!(bigger_rectangle.can_contain(&smaller_rectangle));
    }

    #[test]
    fn rectangle_can_contain_another_one_if_same_size() {
        let bigger_rectangle = Rectangle {
            width: 20,
            height: 20
        };

        let smaller_rectangle = Rectangle {
            width: 20,
            height: 20
        };

        assert!(bigger_rectangle.can_contain(&smaller_rectangle));
    }

    #[test]
    fn rectangle_cannot_contain_another_one_if_exceeds_width() {
        let bigger_rectangle = Rectangle {
            width: 20,
            height: 20
        };

        let smaller_rectangle = Rectangle {
            width: 21,
            height: 20
        };

        assert!(!bigger_rectangle.can_contain(&smaller_rectangle));
    }

    #[test]
    fn rectangle_cannot_contain_another_one_if_exceeds_height() -> Result<(), String> {
        let bigger_rectangle = Rectangle {
            width: 20,
            height: 20
        };

        let smaller_rectangle = Rectangle {
            width: 20,
            height: 21
        };

        if !bigger_rectangle.can_contain(&smaller_rectangle) {
            return Ok(());
        }
        else {
            return Err(String::from("Expected bigger rectangle not to contain smaller"));
        }
    }
}
