fn main() {
    // slice does not have ownership
    // refer contiguous sequence of elements in a collection

    let mut s = String::from("Hello, World");
    let word = first_word_1(&s);
    s.clear();  // this empties the string;
    println!("the first word {}", word);
    // this value of word have no relation to s

    s = String::from("Hello, World");
    let word = first_word_with_slice(&s);
    // s.clear();  // error
    println!("The first word {}", word);
    // ensures that the word is useless!! [ Kind of ]

    // word is still valid, but in reference to s it is not valid
    // using word along with s won't be a good idea

    let s = String::from("Hello, World");
    let hello = &s[0..5];  // of type &str
    let world = &s[7..12];
    println!("{} {}", hello, world);

    let slice = &s[..4];
    let slice = &s[4..];
    let slice = &s[..];  // equivalent to &s[0..s.len()]

    let s = String::from("Hello, World");
    let word = first_word_with_slice_with_better_fun_sig(&s[..]);
    let string_literal = "Hello, World";
    let word = first_word_with_slice_with_better_fun_sig(
        &string_literal
    );

    // slices can be applied to list...
    let a = [1, 2, 3, 4];
    let slice = &a[1..3];
    // a[1..3] does not work
}

fn first_word_1(s: &String) -> usize {
    // take a string return the last index of the word
    let bytes=s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_with_slice(s: &String) -> &str {
    let bytes=s.as_bytes();
    for (i, &item) in bytes.iter().enumerate()  {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word_with_slice_with_better_fun_sig(s: &str) -> &str {
    // the logic remains the same
    // can be called as first_...fun_sig(&st[..]) if st is a string literal
    let bytes=s.as_bytes();
    for (i, &item) in bytes.iter().enumerate()  {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}