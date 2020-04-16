#![allow(unused_variables)]
pub mod mod_iterator {
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
        println!("v2 {:?}", v2)

        // NEXT: Using Closures that Capture Their Environment
    }
}
