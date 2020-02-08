fn main() {}

// Note: you can look up multiple way to configure test to:
// - limit amount of CPU cores
// - run a specific amount of tests
// - allow print statements to be shown during tests
// - etc..

// Note: you can create a folder next to src called test if you want to better separate them.
// - Look up integration tests.
// - the tests folder is considered its own separate crate.

// private & public functions are allowed to be tested
fn test_even(x: i32, y: i32) -> bool {
    x == y
}

// tests can be run with the `cargo test` command.
// this test can be called with `cargo test --bin ch11_automated_tests`
#[cfg(test)] // this annotation tells the compiler to only compile this module during cargo tests
mod tests {

    use super::*; // import outside function

    // this attribute indicates a test function
    #[test]
    fn it_works() {
        // this macro checks if a condition is true
        assert!(100 > 0);
        assert!(!(0 > 100));

        // this macro checks id 2 values are the same
        assert_eq!(2 + 2, 4);

        // this marco is the opposite of assert_eq!
        assert_ne!(12, 2000);

        // you can add messages to your errors
        let some_var = ". oh no";
        assert!(true, "this won't print since it passed {}", some_var);
    }

    // tests can be set to only pass on a panic
    #[test]
    #[should_panic]
    fn exploration() {
        panic!("I never had a chance of passing :(");
    }

    // panics can be filtered for a specific message
    // this ensures the type of error was expected
    #[test]
    #[should_panic(expected = "stuck at home")]
    fn adventure() {
        panic!("stuck at home");
    }

    // results can be used to determine if a test passed/failed
    #[test]
    fn exam() -> Result<(), String> {
        Ok(())
    }

    // you can bring in functions out of scope
    #[test]
    fn outside_fun() {
        assert!(test_even(2, 2));
    }
}
