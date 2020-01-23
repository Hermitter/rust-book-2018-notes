fn main() {
    // Mutability
    let mut x = 25;
    x = x + 25;
    println!("x is now: {}", x);

    // Shadowing
    let x = 5;
    println!("The value of x is: {}", x);

    let x = x.to_string() + " as a string";
    println!("The value of x is: {}", x);

    // Tuples
    let x: (i32, f64, u8) = (500, 6.4, 1);
    println!("Set of nums: {}, {}, {}", x.0, x.1, x.2);

    let (x, y, z) = x;
    println!("Same set of nums {}, {}, {}", x, y, z);

    // Arrays
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; //  let a = [3, 3, 3, 3, 3];

    // Functions
    simple_function();
    argument_in_function(55);
    let x = returns_in_functions(5);

    // Statement
    let x = 4;

    // Expression
    let y = {
        let x = 3;
        x + 23
    };

    // If Statements
    let num = 3;
    if num < 5 {
        println!("num was less than 5");
    } else if num == 5 {
        println!("num is exactly 5");
    } else {
        println!("num was greater than 5");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    // Loop Loops
    loop {
        println!("I can run forever, but I won't");
        break;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 5 {
            break "loop can return a value!";
        }
    };
    println!("{}", result);

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // While Loops
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // For Loop
    for x in a.iter() {
        println!("For loop found {}", x);
    }
}

fn simple_function() {
    println!("A simple function");
}

fn argument_in_function(x: i32) {
    println!("The value you gave was: {}", x);
}

fn returns_in_functions(x: i32) -> i32 {
    x + 50
}
