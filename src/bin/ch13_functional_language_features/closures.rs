// closures are anonymous functions that can capture values from
// the scope of which they're defined.

use std::thread;
use std::time::Duration;

pub fn run() {
    println!("testing");

    // # Example
    // closures are allowed to be stored in functions
    // they're usually used in a narrow context
    let double_a_value = |num| {
        println!("calculating slowly...");
        // thread::sleep(Duration::from_secs(2));
        num
    };

    // they can be called later
    let some_number = double_a_value(23);

    // # closure syntax examples
    // - define input & return type
    let add_one_v1 = |x: u32| -> u32 { x + 1 };
    // - parameter & return types are inferred at compile time. Note that closure won't have multiple signatures (just one)
    // - Curly braces are optional for single expressions.
    let add_one_v2 = |x| x + 1;

    // again, closures are able to be invoked later
    let result = add_one_v2(23);

    // # structs can hold closures
    // each closure must be defined as a separate generic type.
    // regardless of the signature, each closure must be explicitly defined.
    struct MasterAdder<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        result: Option<u32>,
    }

    // struct closures can be called from it's methods
    impl<T> MasterAdder<T>
    where
        T: Fn(u32) -> u32,
    {
        fn get_result(&mut self, arg: u32) -> u32 {
            let result = (self.calculation)(arg);
            self.result = Some(result);
            result
        }
    }

    // closure are easily passed to structs, similar to variables
    let mut add_master = MasterAdder {
        calculation: |x| x + 50,
        result: None,
    };

    println!("MasterAdder result: {}", add_master.get_result(30));

    // # closures have access to the scope of where they're defined.
    // for example, using the value x which was defined outside the closure.
    // note, capturing values like this will waste more memory.
    let x = 4;
    let equal_to_x = |z: i32| z == x;

    assert!(equal_to_x(4));

    // # types of closure traits
    // - FnOnce: consumes the variables it captures from its enclosing scope, known as the closure’s environment. To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined. The Once part of the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only once.
    // - FnMut: can change the environment because it mutably borrows values.
    // - Fn: borrows values from the environment immutably.

    // # closures can take ownership of variables
    // same example as before, but the move keyword is used
    let x = 4;
    let equal_to_x = move |z: i32| z == x;

    // println!("x has been moved!!!: {:?}", x); // this line would crash the program

    assert!(equal_to_x(4));
}
