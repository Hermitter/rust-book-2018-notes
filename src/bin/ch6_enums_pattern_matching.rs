fn main() {
    enums();
    match_operator();
    if_let();
}

fn enums() {
    // Creating an enum
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    let user1 = IpAddrKind::V4;
    let user2 = IpAddrKind::V6;

    // Using an enum in a function
    fn route(ip_type: IpAddrKind) {
        println!("{:?}", ip_type);
    }

    route(user1);
    route(user2);

    // Storing values in an enum
    enum IpAddr {
        V4(String),
        V6(String),
        V7(u8, u8, u8, u8),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopBack = IpAddr::V6(String::from("::1"));

    // Example of enums returning different struct
    struct Ipv4Addr {
        // --you get the gist--
    }

    struct Ipv6Addr {
        // --you get the gist--
    }

    enum IpAddrress {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    // structs are handy because the definitions are all under one name.
    // This makes it easier for function inputs.
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // Enums can have methods
    impl Message {
        fn call(&self) {
            // if let will be covered later
            if let Message::Write(txt) = &self {
                println!("{}", txt);
            }
        }
    }
    let msg = Message::Write(String::from("hello"));
    msg.call();

    // The Option Type
    // Rust's way of handling null values
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
    let absent_string: Option<String> = None;
}

fn match_operator() {
    // match example
    enum Coin {
        HayPenny,
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => {
                println!("You can have functions run before a return");
                10
            }
            Coin::Quarter => 25,
            _ => 0, // default
        }
    }
    println!("{}", value_in_cents(Coin::Quarter));

    // Extracting enum data
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum UsCoin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    let coin = UsCoin::Quarter(UsState::Alabama);
    let x = match coin {
        UsCoin::Penny => 1,
        UsCoin::Nickel => 5,
        UsCoin::Dime => 10,
        UsCoin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    };

    // Matching with Option<T>
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn if_let() {
    // best use for match one patterns
    let some_u8_value = Some(0u8);

    if let Some(3) = some_u8_value {
        println!("three");
    }

    // Enum if let example
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    let coin = Coin::Quarter(UsState::Alabama);

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
