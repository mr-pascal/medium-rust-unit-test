fn main() {
    println!("Hello, world!");
}

fn adder(a: i64, b: i64) -> i64 {
    return a + b;
}

fn subtract() -> i64 {
    // TODO: logic not yet implemented
    return 4;
}

#[cfg(test)]
mod tests {
    use crate::{adder, subtract};

    #[test]
    fn add_numbers() {
        assert_eq!(adder(2, 2), 4);
    }

    #[test]
    #[should_panic(expected = "Make this test fail")]
    fn failing_test() {
        panic!("Make this test fail");
    }

    #[test]
    #[ignore]
    fn subtract_numbers() {
        // This test is ignored for now
        assert_eq!(subtract(), 6);
    }
}
