mod middle_of_house;
mod backyard_of_house;

pub use middle_of_house::middle_ware;
use backyard_of_house::backyard;

pub mod front_of_house {
	pub mod hosting {
		pub fn add_to_waitlist() {}
		fn seat_at_table() {}
	}

	pub mod serving {
		fn take_order() {}
		pub fn serve_order() {}
		fn take_payment() {}
	}

}

mod back_of_house {

	#[derive(Debug)]
	pub enum Appetizer {
		Soup,
		Salad,
	}

	#[derive(Debug)]
	pub struct Breakfast {
		pub toast: String,
		seasonal_fruit: String,
	}

	impl Breakfast {
		pub fn summer(toast: &str) -> Breakfast {
			Breakfast {
				toast: toast.to_string(),
				seasonal_fruit: String::from("Peaches"),
			}
		}
	}

	fn fix_incorrect_order() {
		cook_order();
		super::front_of_house::serving::serve_order();
	}

	fn cook_order() {}

	fn another_using_super() {
		super::dummy_01();
	}
}

fn dummy_01(){println!("dummy_01");}

pub fn eat_at_restaurant() {
	println!("eat_at_restaurant");
	crate::front_of_house::hosting::add_to_waitlist();

	front_of_house::hosting::add_to_waitlist();

	let mut meal = back_of_house::Breakfast::summer("Rye");
	println!("I'd like {} toast please", meal.toast);
	
	meal.toast = "Wheat".to_string();
	println!("I'd like {} toast please", meal.toast);

	let order1 = back_of_house::Appetizer::Salad;
}

pub fn main() {
	eat_at_restaurant();
	middle_ware::middle();

	println!("Checking backyard");
	backyard::backyard();
}