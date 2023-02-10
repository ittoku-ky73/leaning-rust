pub enum Appetizer {
    Soup,
    Salad,
}

pub struct Breakfast {
    pub toast: String,
    _seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            _seasonal_fruit: String::from("peaches"),
        }
    }
}

fn _fix_incorrect_order() {
    _cook_order();
}

fn _cook_order() {}
