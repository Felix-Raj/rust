#![allow(unused_variables)]
pub mod mod_iterator {
    struct Counter {
        count: i32,
    }
    impl Counter {
        fn new() -> Counter {
            Counter { count: -1 }
        }
    }

    impl Iterator for Counter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }
    pub fn iterators() {
        // iterators are lazy
        let v1 = vec![1, 2, 3];
        let iter = v1.iter(); // does not nothing until consumed

        // consume iter
        for x in iter {
            println!("Got: {}", x);
        }

        // iterators implement Iterator trait which is as
        // pub trait Iterator {
        //      type Item;
        //      fn next(&mut self) -> Option<Self:Item>;
        // }
        // Argument to call should be a mutable reference. In case of a for loop the loop takes the
        // ownership of iterator. Thus in background it is made mutable.
        // Therefor the following will not work,
        /*
        let sum :i32 = iter.sum();
        println!("Sum: {}", sum)
        */

        // consuming adaptors -  methods that call next.
        // sum is an consuming adaptor, it take ownership of the iterator so as to call the next
        // method, thus after call to sum, we cannot use iter anymore.
        let iter = v1.iter();
        let sum: i32 = iter.sum();
        println!("Sum: {}", sum);

        // iterator adaptors - methods which produces iterators
        // Multiple call to iterator adaptors can be chained, like map below.
        // This is also lazy.
        // collect method can be used to collect the results
        let iter = v1.iter().map(|x| x * x);
        let v2: Vec<_> = iter.collect();
        println!("v2 {:?}", v2);

        // Closures that capture the environment can also be used
        let divisible_by = 8;
        let v1 = vec![1, 4, 8, 12, 16, 20, 24, 28, 32];
        let v2: Vec<_> = v1.iter().filter(|x| **x % divisible_by == 0).collect();
        println!("v2 {:?}", v2);

        // Creating an iterator, by implementing the Iterator trait
        let c = Counter::new();
        for x in c {
            println!("Next {}", x)
        }

        // A complex one.
        let x1: i32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        println!(
            "Sum of some {}",
            x1
        )

        // iterators, although a high-level abstraction, get compiled down to roughly the same
        // code as if you’d written the lower-level code yourself. Iterators are one of Rust’s
        // zero-cost abstractions, by which we mean using the abstraction imposes no additional
        // runtime overhead.
    }
}
