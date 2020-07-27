/*
Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:
    Rc<T> enables multiple owners of the same data;
        Box<T> and RefCell<T> have single owners.
    Box<T> allows immutable or mutable borrows checked at compile time;
        Rc<T> allows only immutable borrows checked at compile time;
        RefCell<T> allows immutable or mutable borrows checked at runtime.
    Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside
        the RefCell<T> even when the RefCell<T> is immutable.
*/
#[allow(unused_variables)]
pub mod mod_box {
    // written as Box<T>
    // data on heap, the pointer on stack.
    // No performance overhead.
    // Used in the scenarios:
    //      1. We have a type whose size can't be know at compile time, but require the value in a
    //          context that require an exact size.
    //      2. When you have a large amount of data and you want to transfer ownership but ensure
    //          the data won’t be copied when you do so
    //      3. When you want to own a value and you care only that it’s a type that implements a
    //          particular trait rather than being of a specific type

    fn cons_list() {
        // Boxes are useful which recursive types.
        // Recursive types are the types whose size can't be know at compile time.
        // A type can have itself as a part of it. The recursion can be theoretically infinite.
        println!("the cons type");
        // enum List {
        //     Cons(i32, List),
        //     Nil
        // }
        // the above will fail as rust don't know how much memory space will be required to store
        // the enum List
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }
        // now the size of Box is know( which is the size of a pointer ) so the compiler can
        // determine the space required.
        let list = List::Cons(
            1,
            Box::new(List::Cons(
                2,
                Box::new(List::Cons(3, Box::new(List::Cons(4, Box::new(List::Nil))))),
            )),
        );
        // useful in scenario 1
    }

    pub fn box_main() {
        let b = Box::new(5);
        // When b goes out of scope, both the pointer and the data gets deallocate.
        println!("b = {}", b);

        cons_list()
    }
}

pub mod using_deref {
    use std::ops::Deref;

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
            // returns the reference to the value
            &self.0
        }
    }

    // Deref coercion is a convenience that Rust performs on arguments
    // to functions and methods. Deref coercion converts the type that
    // implements the Deref trait to another type. Deref coercion
    // happens automatically when we pass a reference to a particular
    // type’s value as an argument to a function or method that
    // doesn’t match the parameter type in the function or method
    // definition. A sequence of calls to the deref method converts
    // the type we provided into the type the parameter needs.
    // coercion converts &String to &str, as String implements Deref
    // trait such that it returns &str.
    fn coercion() {
        fn hello(name: &str) {
            println!("Hello {}", name);
        }
        // Deref works with immutable reference.
        // DerefMut works with mutable reference.
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
        let n = "another rust".to_string();
        hello(&n);

        // Deref coercion works in the following scenarios also:
        //      from &T to &U, when T: Deref<Target=U>
        //      from &mut T to &mut T, when T: DerefMut<Target=U>
        //      from &mut T to &U, when T: Deref<Target=U>
        // Think like the to as function parameter as the from as
        // function argument.
    }

    pub fn deref_main() {
        let x = 5;
        let y = MyBox::new(5);
        assert_eq!(5, x);
        // y.deref is automatically called
        assert_eq!(5, *y);
        coercion()
    }
}

#[allow(unused_variables)]
pub mod using_drop {
    // Customize what happens when a value goes out of scope.
    struct CustomerPointer {
        data: String,
    }

    impl Drop for CustomerPointer {
        fn drop(&mut self) {
            println!("Dropping customer {}", self.data);
        }
    }

    pub fn drop_main() {
        let c1 = CustomerPointer {
            data: String::from("customer one"),
        };
        let c2 = CustomerPointer {
            data: String::from("customer two")
        };
        println!("Created customers");
        // after going out of scope, c2 gets dropped first, then c1.
        // Drop cannot be disabled. But can be called earlier.
        // Cannot manually call drop method. Should use std::mem::drop
        drop(c1);
    }
}

#[allow(unused_variables)]
pub mod reference_counter {
    use std::rc::Rc;

    // Rc<T>
    // when we need multiple owner for same data
    // keeps track of number of reference of the data, when reference is 0, it cleans up the data
    // only be used with single threaded
    fn not_working() {
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }
        let a = List::Cons(40, Box::new(List::Cons(100, Box::new(List::Cons(0, Box::new(List::Nil))))));
        let b = List::Cons(45, Box::new(a));
        // let c = List::Cons(89, Box::new(a)); // cannot do, a already moved to b, cannot move again
    }

    fn using_rc() {
        enum List {
            Cons(i32, Rc<List>),
            Nil,
        }
        let a = Rc::new(List::Cons(0, Rc::new(List::Cons(1, Rc::new(List::Nil)))));
        println!("count after a {}", Rc::strong_count(&a));
        let b = List::Cons(-1, Rc::clone(&a));
        println!("count after b {}", Rc::strong_count(&a));
        let c = List::Cons(-2, Rc::clone(&a)); // fine
        // using Rc::clone does not produce a deep copy as other clones, it only increases the reference
        // counter. So it's faster than normal clones
        println!("count after c {}", Rc::strong_count(&a));

        // to see how scope affects reference counter
        {
            let d=List::Cons(-3, Rc::clone(&a));
            println!("count after d {}", Rc::strong_count(&a));
        }
        println!("count after d went out of scope {}", Rc::strong_count(&a));
    }

    pub fn main() {
        not_working(); // panics
        using_rc();
    }
}

#[allow(unused_variables)]
pub mod ref_cell {
    // interior mutability - allows to mutate data even if there are immutable references to the
    // data. Not allowed by the borrow rules normally.
    // the pattern use unsafe code inside a data structure to bend the borrow rules. unsafe code
    // involved is then wrapped in a safe API, outer type remains immutable.
    // the compiler is conservative, it will reject any program that it find (via analysis) cannot
    // be compiler with borrow rule. it's impossible to analyze correctness of all programs. but it
    // will reject any it cannot give a green flag. if the author is sure that the program satisfies
    // the borrow rule, but compiler rejects because it cannot verify this then RefCell will help.
    // only be used with single threaded
}