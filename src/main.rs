fn main() {
    let x = if true {
        // y is known only inside this if -- scope of y
        let y = 10;
        y+10
    } else {
        0
    };
    println!("{}",x);

    // ownership
   {
       let mut s = String::from("Hello");  // variable gets a
       // space in heap rather than on stack as it can grow/shrink.
       s.push_str(", World!");
       println!("{}",s)
   }  // rust calls the drop function for the variable at this point, the
    // function implements how to return the memory back to OS.

    let x = 5;
    let y = x;
    // bind 5 to x, make a copy of x and bind to y
    // the data is stored to stack rather than heap as the size is known and
    // does not change

    let mut x = 10;
    let mut y = x;
    println!("x and y {} {}", x, y);

    y += 1;
    println!("x and y {} {} after changing y", x, y);
    // x is not affected

    let s1 = String::from("hello");
    let s2 = s1;
    /* string consist of three parts ( in Rust ) -
        1. pointer to the data (the word "hello") in heap
        2. length os string
        3. capacity
       On assigning s2 with s1, this data is copied, so the pointer for
       both the variables point to the same location. Rust does not copy the
       heap data.
    */
    // when both goes out of scope both will try to free
    // memory ( which is a bug ), so Rust instead of copying the memory it
    // considers s1 to be invalid. try

    //println!("s1 {}",s1);

    // this is like a shallow copy, but as the variable is invalidated, it is
    // called *move*.

    // Rust never created *deep copy* by default.
    let s1 = String::from("Hello");
    let s2 = s1.clone();  // this copies heap data too.
    println!("s1 {} s2 {}", s1, s2);

    let x = 5;
    let y = x;
    // here there is no change in deep and shallow copy, so both are valid
    // after the copy

    // if the variable have a Copy trait then the variable is still usable after
    // assignment. We cannot annotate a type with Copy if the type or any
    // part of it have implemented the Drop trait.

    // move - copy applies to function arguments and returning as well
    let s1=gives_ownership();
    // the return value is *moved* to s1
    let s2=String::from("Hello");
    let s3=takes_and_gives_back(s2);
    // s2 is moved to fn argument a_string, return is moved to s3
    // as s2 is moved the following throws error
    // println!("s2 {}",s2);
    let x = 10;
    makes_copy(x);  // x is only copied
    println!("x {} valid after being used to call fn", x);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    some_string  // Note - no `;`
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn makes_copy(nu: i32) {
    println!("{}", nu);
}