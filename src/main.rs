fn main() {
    println!("Hello, World!");

    // expression - evaluates to a value
    // statements - do not return a value
    // let x=4 is an expression, while 4 is a statement. Statement 4 returns
    // 4, while let x=4 returns nothing. Thus we cannot do let y = (let x = 4)
    let y = {
        let x= another_function(4);
        x + 1
    };
    // try with let y = { let x = 3; x+1;}
    // with the ending `;` this turns to be a statement which cannot be
    // assigned to a variable
    println!("y is {}", y);
}

fn another_function(x: i32) -> i32 {
    println!("Value if x is {}", x);
    return x + 1;
    // x + 1 is also valid, notice the missing `;`, as this should be an
    // expression rather than a statement;
}