fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("Hello");
    change_mut(&mut s);
    // cannot call change_mut(&mut s1) as s1 is immutable.
    // Variable should be mutable, reference should be mutable.
    println!("Mutable {}", s);

    let e1 = &mut s;
    let e2 = &mut s;
    // can have only one mutable reference to a particular piece of data in
    // a particular scope, the following will fail
    // println!("e1 {} e2 {}", e1, e2);
    let e1 = &mut s;
    {
        let e2 = &mut s;
        // Does not work
        // println!("e1 {} e2 {}", e1, e2);
    }  // e2 moves out of scope here
    // so
    let e2 = &mut s;  // is fine

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
    // cannot have a mutable reference when we have immutable references, print
    // will not work
    // println!("{}, {}, and {}", r1, r2, r3);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    println!("{} and {}", r1, r2);  //no probs
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // Dangling References
    // let ref_to_nothing = dangle();

    /*

    At any given time, you can have either one mutable reference or
        any number of immutable references.
    References must always be valid.

    */

}

fn calculate_length(s: &String) -> usize {
    // refer the similar fun in functions

    // s gets reference to the string being passed ( that contain the pointer
    // to the mmr location containing the data, len of str, capacity of str).
    s.len()
}  // s goes out of scope, but memory is not freed as s does not have
// ownership over the parameter, it have just a reference.

fn change(some_string: &String) {
    // as some_string is *borrowed*, we cannot modify it.
    // some_string.push_str(", World");
}

fn change_mut(some_string: &mut String) {
    // here the reference is mutable as stated in the argument.
    some_string.push_str(", World");
}

fn dangle() -> &String {
    let s = String::from("Hello");
    &s  // should return the string instead
}  // s goes out of scope so calling guy gets a pointer to
// invalid/non-existent data, this is prevented by the compiler