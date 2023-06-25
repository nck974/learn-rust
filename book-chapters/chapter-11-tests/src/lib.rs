// Expose function to be used by the integration tests
pub fn add_two(num: i32) -> i32 {
    num + 2
}

#[cfg(test)]
mod tests {
    // use std::time::Duration;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    #[ignore]
    fn it_does_not_work() {
        let result = 3 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn it_crashes() {
        panic!("Intentional crash");
    }

    // Example using assert with objects
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    #[test]
    fn rectangle_can_hold_other_rectangle() {
        let rect = Rectangle {
            width: 25,
            height: 23,
        };
        assert!(rect.can_hold(&Rectangle {
            width: 5,
            height: 2
        }));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    // Expect a paninc
    #[should_panic(expected = "Expected")] // Specific info on what crashes
    #[test]
    fn expected_panic() {
        // std::thread::sleep(Duration::from_millis(3000));
        panic!("Expected");
    }

    #[test]
    fn it_works_with_result() -> Result<(), String> {
        // std::thread::sleep(Duration::from_millis(3000));
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
