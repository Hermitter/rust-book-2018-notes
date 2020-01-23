fn main() {
    // Collections are stored on the heap
    vectors();
    strings();
    maps();
}

fn vectors() {
    // Vectors are growable lists that hold 1 type
    let mut v: Vec<i32> = Vec::new();

    // There is a marco in the std lib that can easily create vectors & infer the type
    let mut v = vec![0, 1, 2, 3];

    // Adding to a vector
    v.push(4);
    v.push(5);
    println!("{:?}", v);

    // Removing from a vector
    v.pop();

    // vectors out of scope are gone along with its elements
    {
        let v = vec![1, 2, 3, 4];
    }

    // 2 ways to read a vector's value.
    let val: &i32 = &v[2];

    match v.get(2) {
        Some(val) => println!("The third element is {}", val),
        None => println!("There is no third element."),
    }

    // Out of bounds
    match v.get(100) {
        None => println!("This index doesn't exist"),
        _ => (),
    }

    // Borrow checker reminder (immutable & mutable references cannot be in the same scope)
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    println!("This is okay: {}", first);

    v.push(6); // this counts as a mutable reference

    // println!("This is not okay: {}", first);

    // Iterating though a vector
    let mut v = vec![30, 15, 60];
    for i in &v {
        println!("{}", i);
    }

    // Iterating & changing the values of a vector
    for i in &mut v {
        *i += 50; // the star dereferences that value so we can change it
        println!("{}", i);
    }

    // Example of dereferencing a value
    let num = 20;
    let new_num = &num;
    println!("{}", *new_num + 5);

    // Storing different value type in a vector
    #[derive(Debug)]
    enum Id {
        // enum types must be known at runtime
        Code(i32),
        Name(String),
    }

    let list = vec![
        Id::Name("Blake".to_string()),
        Id::Code(004442),
        Id::Code(005552),
    ];

    println!("{:?}", list);
}

fn strings() {
    // Creating a new string;
    let mut string = String::new();
    let string2 = String::from("hello!");

    // String from string literal
    let str_example = "hello world!";
    let string_example = str_example.to_string();

    // utf-8 examples
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Ways of appending to a string
    let mut word = String::from("Hello ");
    word.push_str("World!");
    word += "How're you?";

    // Appending without a reference will cause a move
    let word1 = String::from("Hello ");
    let word2 = String::from("World");
    let word3 = word1 + &word2; // word1 was moved and can no longer be used
    println!("{}", word3);

    // format!() is used when complicated strings need to be expressed
    // No ownership is taken when using this!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3); // tic-tac-toe

    // Indexing characters in a string
    // Know that String is a wrapper over a Vector
    // - you can take a slice (not ideal because this does not equate to a character in other langs नमस्ते )
    let text = "Hello To All My Friends I Don't Have";
    let word = &text[0..5];
    println!("{}", word);

    // - iter through each char
    for c in text.chars() {
        println!("{}", c);
    }

    // - iter through each byte
    for b in text.bytes() {
        println!("{}", b);
    }

    // Remember that no all letters in a language are one byte
    println!("{}", "नमस्ते".len()); // 18 bytes
}

use std::collections::HashMap;

fn maps() {
    // Creating a new hash map
    let mut scores = HashMap::new();
    // Inserting values into map
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Hash maps can also be made from a vector of tuples
    let teams = vec![(String::from("Blue"), 10), (String::from("Yellow"), 50)];
    let scores: HashMap<_, _> = teams.into_iter().collect(); // <_,_> will be covered later, but know that it tells the compiler to figure out the hash map's types
    println!("{}", scores["Blue"]);

    // Types that don't implement the Copy trait are moved into the map
    let text = String::from("Blue");

    let mut scores = HashMap::new();
    scores.insert(0, text);
    // println!("{}", text);// will crash. value has been moved

    // Obtaining values from a map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 10);

    let score = scores.get(&"Blue".to_string());
    println!("score: {}", score.unwrap());

    // Iterating through a map
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Values can be overwritten
    scores.insert(String::from("Blue"), 25); // blue went from 10 -> 25

    // Check if value exists before inserting
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(55); // does not set value

    // Updating a map value Based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // or_insert returns a mutable reference to the map value
        *count += 1;
    }

    println!("{:?}", map);
}
