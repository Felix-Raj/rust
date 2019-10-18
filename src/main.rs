fn main() {
    let number = 3;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // condition *must be* a bool
    /*if number {
        // wont compile
    }*/

    let number = if true {
        number + 2  // no semi-colon!!
    } else {
        number
        // "six" // <-- Error!!!, all arms should be of same type
    };
    // when using if to assign value to an variable, each arm should evaluate
    // to compatible data type

    println!("number is {}", number);

    let mut ctr = 0;
    loop {
        println!("Counter value {}", ctr);
        ctr += 1;
        if ctr >= 4 {
            break;
        }
    }

    ctr = 0;
    // do operations that may fail repeatedly
    let result = loop {
        ctr += 1;
        if ctr == 10 {
            break ctr * 2;
            // or break ctr * 2
        }
    };
    println!("Result {}", result);

    ctr = 0;
    while ctr != 4 {
        println!("ctr {}", ctr);
        ctr += 1;
    };
    // assignment to variable as in loop does not work here, also break does
    // not work here. Should be like
    ctr = 0;
    while ctr != 4 {
        println!("ctr wb {}", ctr);
        ctr += 1;
        if ctr == 2{
            // break ctr; //  <- does not work
            break;
        }
    };

    let a:[i32;4] = [10, 20, 12, 34];
    for x in a.iter() {
        println!("The value is {}",x);
    };

    // range
    for y in 1..10 {
        println!("The value of y {}", y);
        println!("fib({}) = {}", y, fib(y));
    }
}

fn fib(pos: i32) -> i32 {
    let mut a: i32 = 1;
    let mut b: i32 = 1;
    let mut c: i32 = 0;
    let mut pos = pos;
    if pos == 1 {
        return a;
    }
    while pos > 1 {
        c = a;
        a = b;
        b = c + b;
        pos -= 1;
    }
    return a;
}
