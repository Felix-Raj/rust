const MAX:u32 = 1000;
// constants - use const instead of let.
// should specify the type.
// can be declared in any scope.
// can only be set to a constance expression, not to result of function call or any other value
// that can be computed at run time.
fn main() {
    let x=5;
    println!("Value of x is {}", x);
    // x = 6;  // x in immutable, cannot reassign

    let mut y = 56; // mutable can be changed
    println!("Value of y is {}", y);
    y = 12;
    println!("Value of y is {}", y);

    // immutable can be shadowed
    let x = x+MAX;
    // can change the type when shadowing [see master-guessing game]
    // as using let, it creates another variable
    println!("[Shadowing] The value of x is {}", x);
    // mutable variable cannot be shadowed
    // y = 100;  // fine, with warnings
    // y = "felix";  // mismatched types error
}
