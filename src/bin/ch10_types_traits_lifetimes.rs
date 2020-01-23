use std::clone;
use std::fmt::Debug;
use std::fmt::Display;

fn main() {
    generics();
    traits();
    lifetimes();
}

fn generics() {
    // generic struct values (convention is to use T as the name)
    // both x & y must share the same type because only one generic value (T) is defined
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("int: {:?}, float: {:?}", integer, float);

    // multiple types can be defined by adding more generic variables
    struct MorePoints<T, U> {
        x: T,
        y: U,
    }

    let mixedNums = MorePoints { x: 5.23, y: 10 }; // x & y can be different values now

    // enums can also use generics to hold values. Similar to Option and Result
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // generic methods for generic structs
    impl<T> Point<T> {
        fn simpleRef(&self) -> &T {
            println!("GENERICS, YEAHHHHHHHH");
            &self.x
        }
    }

    let x = Point { x: 20, y: 34 };
    x.simpleRef();

    // methods for specific type in generic struct
    impl Point<i32> {
        fn wholeNums(&self) {
            println!("WHOLE NUMS ONLY x:{} y:{}", self.x, self.y);
        }
    }

    x.wholeNums();
}

fn traits() {
    // traits are mainly defined by signature
    // each implementation must explicitly defined
    trait Summary {
        fn summarize(&self) -> String;

        // however traits can also have default methods
        fn preview(&self) -> String {
            String::from("(Read more...)")
        }
    }

    // summary trait implemented for a Struct
    // this is similar to creating methods
    struct Tweet {
        username: String,
        content: String,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let sample_tweet = Tweet {
        username: String::from("Cazzy"),
        content: String::from("hey look at me birthing into to the world of twitter"),
    };

    println!("{}", sample_tweet.summarize());
    println!("New review released: {}", sample_tweet.preview());

    // traits can be used as parameters
    fn notify(item: impl Summary) {
        println!("Breaking news! {}", item.summarize()); // trait methods can the be called
    }

    notify(sample_tweet);

    // traits can be used as parameters (no syntax sugar)
    // this allows us to specify parameters with the same type
    fn notify2<T: Summary>(item1: T, item2: T) {
        println!("{} {}", item1.summarize(), item2.preview());
    }

    // "where" syntax allows you to specify multiple traits easily
    fn some_function<T, U>(t: T, u: U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        return 0;
    }

    // a parameter specify multiple traits
    fn notify3(item: impl Summary + Display) {}

    // traits can be used as return values
    fn returns_summarizable() -> impl Summary {
        // only one type that implements summary can be used as the return value
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
        }
    }
}

fn lifetimes() {
    // parameter references being returned need to have a generic lifetime specified.
    // similar to defining generics, but lifetimes start with '
    // 'a will have the same lifetime of the smallest lifetime in the parameter.
    // in this case, x and y are the same so it doesn't matter
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    longest("hello", "world");

    // lifetimes don't need to be given to a parameter that's not returned
    fn simpleReturn<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }

    // structs can also contain references by using specifying a lifetime
    struct User<'a> {
        name: &'a str,
        age: i32,
    }

    let user_1 = User {
        name: "Linda",
        age: 24,
    };

    println!("{} is {}", user_1.name, user_1.age);

    // some signature patterns are common enough that rust already creates the lifetimes
    fn some_word(s: &str) -> &str {
        "LOOK AT THIS WORD"
    }

    // there are 3 rules for references in functions
    // 1. each parameter gets its own lifetime
    // 2. the lifetime of the one input parameter gets assigned to the output lifetime,
    // 3. if there are multiple input lifetime parameters, but one of them is self(aka. a method), the lifetime of self is assigned to all output lifetime parameters.

    // structs with lifetimes require lifetimes specified in the impl block
    impl<'a> User<'a> {
        fn some_number(&self) -> i32 {
            3
        }

        fn alert(&self, text: &str) -> &str {
            println!("Attention please: {}", text);
            self.name
        }
    }

    // static lifetimes can be created. they live for the entire duration of the program
    let some_text: &'static str = "I have a static lifetime.";
}
