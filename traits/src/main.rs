use std::fmt::{Debug, Display, Formatter};

trait Drinkable {
    fn consume(&mut self);
    fn get_data(&self) -> String;
    fn stats(&self) {
        println!("{}", self.get_data());
    }
}
#[derive(Debug)]
enum Milk {
    Whole,
    Oat,
    Almond,
}

struct Coffee<T> {
    kind: T,
    milk: Milk,
    ounces: u32,
}

impl<T> Coffee<T> {
    fn new(kind: T, milk: Milk, ounces: u32) -> Self {
        Self { kind, milk, ounces }
    }
}

impl<T: Debug> Debug for Coffee<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Coffee")
            .field("kind", &self.kind)
            .field("milk", &self.milk)
            .field("ounces", &self.ounces)
            .finish()
    }
}

impl<T: Display> Drinkable for Coffee<T> {
    fn consume(&mut self) {
        self.ounces = 0;
    }

    fn get_data(&self) -> String {
        format!("A delicious {} ounce {}", self.ounces, self.kind)
    }
}
// #[derive(Debug)]
struct Soda {
    calories: u32,
    price: f64,
    flavor: String,
    percentage: u8,
}

impl Soda {
    fn new(calories: u32, price: f64, flavor: String) -> Self {
        Self {
            calories,
            price,
            flavor,
            percentage: 100,
        }
    }
}

impl Drinkable for Soda {
    fn consume(&mut self) {
        self.percentage = 0;
    }

    fn get_data(&self) -> String {
        format!("Flavor: {}, Calories: {}", self.flavor, self.calories)
    }
}

impl Display for Soda {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "** {} Soda **", self.flavor)
    }
}

impl Clone for Soda {
    fn clone(&self) -> Self {
        Self {
            calories: self.calories,
            price: self.price,
            flavor: self.flavor.clone(),
            percentage: self.percentage,
        }
    }
}

impl PartialEq for Soda {
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    fn eq(&self, other: &Self) -> bool {
        self.calories == other.calories
            && self.price == other.price
            && self.flavor == other.flavor
            && self.percentage == other.percentage
    }
}

impl Eq for Soda {}
fn main() {
    let mut latte = Coffee::new("latte", Milk::Oat, 32);
    println!("{:?}", latte);
    latte.consume();
    println!("{:?}", latte);

    let cappuccino = Coffee::new(String::from("Cappuccino"), Milk::Oat, 32);
    println!("{:?}", cappuccino);

    let pepsi = Soda::new(300, 29.8, String::from("Cheri"));
    println!("{}", pepsi);

    let mut coke = pepsi.clone();
    println!("{}", pepsi == coke);
    coke.consume();
    println!("{}", pepsi == coke);
}
