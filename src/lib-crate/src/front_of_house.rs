// define module using mod

/* this creates a *module tree* as follows
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
*/

/* To find an item in the module tree we need a path.
Path can be
    (1) absolute path - starts from crate root using crate name or the literal `crate`
    (2) relative path - starts from current module - use `self`, `super` or an
            identifier in the current module

     the identifiers in the path are separated using `::`
*/
pub mod customer;
pub mod front_of_house {
    // can also hold definition of enums, structs,...
    pub mod hosting {  /* making a module public will not make content public, but only
                            allow ancestor to refer the module*/
        /* by default items inside a module are private so cannot use
            crate::front_of_house::hosting::add_to_waitlist() outside the module
            this defines *privacy boundary* and helps encapsulation
        items in parent modules are accessible to children, but children cannot
            access the private items in parent module
        siblings can refer other sibling w/o regards to whether the sibling is
            pub of not, that's how the eat_at_restaurant is able to refer front_of_house.
            As content of a module are not exposed, we need to make hosting module public
            to access it
        */
        // fn add_to_waitlist() {}  // this is private
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }

    pub mod back_of_house {
        // let back of house see everything in front of house
        pub struct Breakfast {
            /* Having struct, enums,.. in the mod makes it private, make it public
                using `pub`, but this doesn't make the fields public, we should
                set the fields as public.
             */
            pub toast: String, // public
            seasonal_fruit: String, // private
        }
        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }

        /* Contrast to struct, all variant of a public enum is public*/
        pub enum Appetizer { Soup, Salad, }

        fn fix_incorrect_order() {
            cook_order();
            super::serving::serve_order();
        }

        fn cook_order() {}
    }
}

// `pub` as the API needs to be exposed.
pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();  // can also use super, see fix_incorrect_order
    // front_of_house is sibling thus can refer

    let mut meal = front_of_house::back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("Give {} toast please", meal.toast);
    // meal.seasonal_fruit = "Cherry".to_string();  // No...No...No!!

    /*
    let mut breakfast = front_of_house::back_of_house::Breakfast {
        toast: "some toast".to_string(),
        seasonal_fruit: "Peach".to_string(),  /*IDE is not flagging error, but this is not
        possible*/
    };
    */

    // all variants of enum will be public for an public enum
    let order1 = front_of_house::back_of_house::Appetizer::Salad;
    let order2 = front_of_house::back_of_house::Appetizer::Soup;

    /* Bring a path into the scope and use items in the scope using `use`. */
    // fine to have o/s the fn
    use front_of_house::back_of_house::Appetizer;  // 1 (absolute??)(2 and 3 are preferred)
    use crate::front_of_house::front_of_house::back_of_house::Breakfast;  // 2 (absolute)
    use self::front_of_house::back_of_house;  // 3 (relative)

    let order3 = Appetizer::Salad;
    let meal2 = Breakfast::summer("lemon");

    use self::front_of_house::hosting::add_to_waitlist;  // this is possible but unidiomatic
    use self::front_of_house::hosting;  // then
    hosting::add_to_waitlist();  // is preferred
    // the exception to this rule is when bringing two items with the same name to
    // the scope
    use std::io;
    use std::fmt;
    /* cannot do
    ```rust
        use std::io::Result;
        use std::fmt::Result;  // the names collide!, rust will cry!
    ```
    */
    /*fn function1 () -> fmt::Result {}
    fn function2 () -> io::Result<()> {}
    // but can use `as`
    use std::fmt::Result;
    use std::io::Result as IoResult;
    fn function3 () -> Result {}
    fn function4 () -> IoResult<()> {}*/
    /* Comment out function1 - function4 as the return value is not compatible with the
    specified type, it will throw error. */

    // Nested paths
    /*
    use std::io;
    use std::cmp::Ordering;

    can be shortened to
    use std::{cmp::Ordering, io};

    Nesting can be done at any level, even when sharing same sub path
    eg:-
    use std::io;
    use std::io::Write;

    can be shortened to
    use std::io::{self, Write};
    brings std::io and std::io::Write into scope
    */

    /*
    * glob operator
    use std::collections::*;
    */

    /* The name brought to the scope using `use` is private to the scope. To
    enable any other (external) code using our code (using use) to use the
    name that we have made available (using `use`) we should *re-export* the
    name using `pub use`.
    This is useful when we want the user to see a different structure than what
    actually the structure is.
    */
}