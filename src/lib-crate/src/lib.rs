mod front_of_house;  // load content of the module from file "front_of_house"

pub use crate::front_of_house::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}