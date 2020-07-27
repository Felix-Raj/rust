use rbox::{mod_box, using_drop, reference_counter};
use rbox::using_deref::deref_main;

fn main() {
    mod_box::box_main();
    deref_main();
    using_drop::drop_main();
    reference_counter::main();
}
