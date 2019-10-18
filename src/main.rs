fn main() {
    // scalar and compound type

    // scalar - single value, int, float, bool, char

    // also see https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow

    let a=2.3;  // f64
    let b: f32 = 2.3;  // f32
    let c = 'x'; // char

    // compound types - multiple values into one type
    // tuples and arrays
    let tup: (i32, f64, u8) = (500, 4.5, 12);
    println!("first element in the tuple is {}", tup.0);
    let (d, e, f) = tup;  // destructuring
    println!("e is {}", e);

    let g = [1,2,3,4,5, 4];  // in stack
    // fixed number of elements
    // vector type can grow or shrink
    // homogeneous, [1,2,3,4,5, 4.6, "Felix", 'f'] not allowed
    // with annotation - g: [i32;6] = [1,2,3,4,5,4];
    println!("array element 0 {}", g[0]);
    let h = [3; 4];  // 3 repeated 4 times
}