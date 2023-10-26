fn main() {
    customer::eat_at_restaurant();    
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        use super::hosting;
        hosting::add_to_waitlist();
    }
}