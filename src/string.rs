// ref: https://doc.rust-lang.org/book/ch08-02-strings.html

pub fn main() {
	/*
	At core of Rust, there is only one string type `str` (string slice 
		type) mostly used in the borrowed form &str.
	String literals (name="name") are also of type str.
	`String` is provided by Rust standard library - it is growable,
		mutable, owned, UTF-8 encoded.
	Using integer index to access characters of strings is not allowed.
		ref https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings
	*/
	
	// New string
	let s = String::new();
	let data = "some data";
	let s = data.to_string();
	let mut s = String::from(data);
	println!("s is now - {}",s);

	// Updating string
	s.push_str(", another data");
	println!("s changed to - {}", s);
	// `push` pushes a single character
	s.push('!');
	println!("s with a character pushed - {}", s);
	// +
	let s1 = String::from("World");
	let s2 = String::from("Hello");
	let s3 = s1 + &s2;
	/*
		s1 will not be accessible here after. The signature of
			the function that does is something like
			add(self, s: &str) -> String {..}
		In our case s2 is borrowed, add function does not take 
			the ownership of s2, so s2 is available
		But ownership of s1 is obtained by add function, so s1
			is no longer available in this scope.
	*/
	println!("s3 - {}", s3);
	// println!("s1 - {}", s1); // will not work
	// format!
	let s1 = String::from("Hello");
	let s4 = format!("{}-{}-{}", s1, s2, s3);
	println!("s4 - {}", s4);

	// slice
	// https://doc.rust-lang.org/book/ch08-02-strings.html#slicing-strings
	let hello = "Здравствуйте";
	let s = &hello[0..4];  // try with 3 or 1
	println!("hello \"{}\" and s \"{}\"",hello, s );

	// iterate
	for c in "नमस्ते".chars() {
    	println!("{}", c);
	}
	for c in "नमस्ते".bytes() {
    	println!("{}", c);
	}
}