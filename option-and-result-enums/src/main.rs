use std::fmt::Error;

#[derive(Debug)]
struct Food {
    name: String,
}
#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        if self.has_mice_infestation {
            return None;
        }

        if self.reservations < 12 {
            return Some(Food {
                name: String::from("Uni Sushi"),
            });
        } else {
            return Some(Food {
                name: String::from("Strip Steak"),
            });
        }
    }

    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            return Err(String::from("Sorry, we have a mice problem"));
        }

        if (address.is_empty()) {
            return Err(String::from("No delivery address specified"));
        }

        return Ok(Food {
            name: String::from("Burger"),
        });
    }
}

fn main() {
    let marios = Restaurant {
        reservations: 11,
        has_mice_infestation: true,
    };
    println!("{:?}", marios.chef_special());
    println!("{:?}", marios.deliver_burger("123 Elm Street"));

    let angelos = Restaurant {
        reservations: 15,
        has_mice_infestation: false,
    };

    println!("{:?}", angelos.chef_special());
    println!("{:?}", angelos.deliver_burger("123 Elm Street"));
}
