// https://doc.rust-lang.org/book/ch08-03-hash-maps.html

use std::collections::HashMap;

pub fn main() {
	/*
	Should be homogenous, all keys are same type; all values are same type
	*/
	let mut scores = HashMap::new();

	scores.insert(String::from("TeamA"), 10);
	scores.insert(String::from("TeamB"), 13);

	let teams = vec![String::from("TeamC"), String::from("TeamD")];
	let i_scores = vec![10, 50];
	let scores: HashMap<_,_> = teams.iter().zip(i_scores.iter()).collect();
	// HashMap<_, _> is needed here because it’s possible to collect into 
	//  many different data structures and Rust doesn’t know which you 
	//	want unless you specify. Here _ is used as the types can be infered.

	// Ownership
	// For types that implement Copy trait, the values are copied to HashMap,
	// 	else the HashMap obtains the ownership
	let mut scores = HashMap::new();
	let team_name = "TeamE".to_string();
	let team_score = 23;
	scores.insert(team_name, team_score);
	scores.insert(String::from("TeamF"), 30);
	scores.insert(String::from("TeamG"), 20);
	// println!("\"team_name\" value is \"{}\"",team_name ); -- does not compile
	// also try with team_name = "TeamE" which is a str rather than String
	let mut hash_map = HashMap::new();
	let team_name = "TeamE".to_string();
	hash_map.insert(&team_name, team_score+10); // is fine

	// Get value
	match scores.get(&team_name) {  // result of get is Result
		Some(expr) => println!("score of TeamE {}", expr),
		None => println!("score of TeamE is None!!"),
	}

	// Iterate
	for (key, value) in &scores {
		println!("{} has score {}", key, value);
	}

	// Update HashMap
	let my_team = "TeamE";
	print_score(&scores, my_team);
	// Overriting 
	scores.insert(String::from("TeamE"), 9);
	print_score(&scores, my_team);
	print_score(&scores, "TeamH");  // new team - TeamH
	// Insert only if no value
	scores.entry(String::from("TeamH")).or_insert(89);
	// Result of entry is an Entry enum,
	// the or_insert mtd in Entry does
	// 		if key exists returns the mutable reference to the value
	//		else insert a new value and returns the mutable reference
	// https://doc.rust-lang.org/beta/rust-by-example/custom_types/enum/testcase_linked_list.html
	print_score(&scores, "TeamH");
	scores.entry(String::from(my_team)).or_insert(1000);
	print_score(&scores, my_team);
	// Update based on old value
	let text = "Hello World World Hello Wonderful";
	let mut map = HashMap::new();
	for word in text.split_whitespace() {
		let count = map.entry(word).or_insert(0);
		// or_insert returns a mutable reference
		*count += 1;
	}
	println!("map {:?}", map);
}

fn print_score(hm: &HashMap<String, i32>, team: &str) {
	match hm.get(team) {
		Some(expr) => println!("Score of team {} is {}", team, expr),
		None => println!("Score of team {} is {}", team, 0),
	}
}
