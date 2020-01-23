fn main() {
    // panic(); not called for obvious reasons
    results();
}

fn panic() {
    // Panics are unrecoverable errors. Ex. out of bounds indexing

    // Rust has a macro to cause a panic
    panic!();

    // They can also have messages
    panic!("I blame Jim for this error.");
}

fn results() {
    // Results are for recoverable errors. Ex. network request returned 404

    // Similar to options, errors can be one of two types
    // Ok(value) / Error(value)
    use std::fs::File;

    let _f = match File::open("Cargo.toml") {
        Ok(f) => f,
        Err(error) => panic!("FILE IS NOT REAL!:{:?}", error),
    };

    // Cleaner examples if you only want to panic
    let _f = File::open("Cargo.toml").unwrap(); // panic
    let _f = File::open("Cargo.toml").expect("FILE IS NOT REAL!"); // panic with message

    // Matching different errors
    use std::io::ErrorKind;

    let _f = match File::open("hello.txt") {
        Ok(f) => f,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f,
                Err(error) => panic!("Could Not Create File: {:?}", error),
            },
            other_error => panic!("Something else went wrong: {:?}", other_error),
        },
    };

    // Cleaner example of handling different error (covered in ch.13)
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Propagating errors
    // ? will automatically return any error received from a function
    // The ? Operator Can Only Be Used in Functions That Return Result
    use std::io;
    use std::io::Read;
    fn return_a_result() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("Not Real.txt")?.read_to_string(&mut s)?; // one of two potential errors can occur here
        Ok(s) // or nothing happens and the desired value is returned
    }

    let _f = return_a_result();

    // simpler example of above
    use std::fs;
    fn return_a_result_simpler() -> Result<String, io::Error> {
        fs::read_to_string("Not Real.txt")
    }
}
