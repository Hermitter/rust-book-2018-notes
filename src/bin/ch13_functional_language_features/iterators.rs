pub fn run() {
    // # Example
    // Iterators allow you to iterate through a list of items
    // Once created, iterators do nothing until they are called.

    let list = vec!["dog", "cat", "cow", "human"];
    let list_iter = list.iter();

    // iterates can be stepped through by using a simple for loop
    for item in list_iter {
        println!("-> {}", item);
    }

    // Vec implements the IntoIterator trait which means that we didn't actually need to call .iter on it.
    for item in &list {
        println!("-> {}", item);
    }

    // # Next method and consuming methods
    // The Iterator trait implements a next() method that manually steps through an iterator by 1.

    // ranges can be turned into iterators.
    // list needs to be mutable because calling .next will consume the iterator as it steps through the values.
    let mut list = (0..10).into_iter(); // into_iter() causes the values to be moved. iter() gives us references
    assert_eq!(list.next(), Some(0));
    assert_eq!(list.next(), Some(1));
    assert_eq!(list.next(), Some(2));
    assert_eq!(list.next(), Some(3));

    // # More methods on iterators

    // the std library contains multiple methods that consume the entire iterator.
    let list_sum: i32 = (0..10).into_iter().sum();
    assert_eq!(list_sum, 45);

    // There are methods make it possible to create an iterator.
    let list: Vec<i32> = vec![1, 2, 3];
    let list2: Vec<_> = list.iter().map(|x| x + 1).collect();

    // # Using closures with iterators
    // closures are commonly used modify the values in an iterator and then return the new collection.
    let numbers = (0..20).into_iter();
    let even_numbers: Vec<_> = numbers.filter(|x| x % 2 == 0).collect();

    assert_eq!(vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18], even_numbers);

    // # Creating your own iterator
    // what's great about implementing your own iterator is that you only need to implement the .next() method
    struct Counter {
        count: u32,
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            // define an end (we don't want an endless iterator)
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    // testing our new iterator
    let mut counter = Counter { count: 0 };
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    // our custom iterator now how access to all the trait methods we've been using: .sum(), .filter(), etc..
    let mut counter = Counter { count: 0 };
    let total: u32 = counter.into_iter().sum();
    assert_eq!(total, 15);
}
