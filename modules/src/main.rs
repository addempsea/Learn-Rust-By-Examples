mod front_of_the_house;
mod my;

use front_of_the_house::hosting;

fn main() {
    hosting::add_to_waitlist();
    my::function();
    my::indirect_access();
    my::nested::function();
}
