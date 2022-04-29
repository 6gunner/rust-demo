mod front_of_house;

use crate::front_of_house::hosting;
use crate::front_of_house::serving;

pub fn eat_at_restaurant() {
    println!("{}", hosting::add_to_waitlist());
    println!("{}", hosting::seat_at_table());
    println!("{}", serving::take_order());
    println!("{}", serving::serve_order());
    println!("{}", serving::take_payment());
}
