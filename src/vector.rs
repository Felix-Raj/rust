// https://doc.rust-lang.org/book/ch08-01-vectors.html

pub fn vector(){
	/*
	More than one value in a single data structure.
	All values next to each other in memmory.
	Values should be of same type.

	As vector goes out of scope - vector is 
		destroyed - destroying the values in it.
	*/

	// new vector
	let mut v1: Vec<i32> = Vec::new();

	// with initial values - using vec! macro
	let v2 = vec![1, 2, 3];  // type annotation not 
							 // required-can be infered

	// update - pushing to vector
	v1.push(1);
	v1.push(10);
	v1.push(7);
	// let mut v = Vec::new(); v.push(2); - will also do
	// here type annotation can be omitted, as can be infered.

	// Get value by index
	println!("The first value {}", &v1[0]);
	// Get by get function
	// get function return Option<&T>
	match v1.get(1) {
		Some(first) => println!("The second value {}",first ),
		None => println!("No value"),
	}
	let element100 = v1.get(100);  // program does not crash
		// but v1[100] will crash

	// Iterate over Vector
	for i in &v1 {
		println!("{}", i);
	}
	for i in &mut v1 {
		*i += 37;
	}
	for i in &v1 {
		println!("{}", i);
	}

	// Store more than one type using enums
	enum SpreadSheetCell {
		Int(i32),
		Float(f64),
		Text(String),
	}
	let v2 = vec![
		SpreadSheetCell::Int(10),
		SpreadSheetCell::Float(10.34),
		SpreadSheetCell::Text("String".to_string()),
	];

	// ownership rules are enforced, the following will crash
	/*
	let first = &v[0];  // immutable borrow
	v.push(4);  // mutable borrow
	println!("Immutable borrow used here again {}", first);
	*/

	// refer standard doc for more operations of vector
}