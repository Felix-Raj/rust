mod front_of_house;  // load content of the module from file "front_of_house"

use crate::front_of_house::front_of_house::hosting;
use crate::front_of_house::customer::customer;
/* Will look as if the module customer is in
front_of_house. This strategy will be useful to refactor
a module. If a module is refactored in such a way that
the module tree remains the same, then the client code
does not get affected.
*/

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    customer::make_order();
}