#![allow(unused_variables)]

use restaurant::{middle_ware,front_of_house::hosting};
fn main() {
	hosting::add_to_waitlist();
	restaurant::main();
	println!("Check pub use", );
	middle_ware::middle();
}