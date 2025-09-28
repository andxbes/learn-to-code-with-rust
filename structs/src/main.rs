#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}
impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Self {
        Self {
            origin,
            destination,
            price,
            passengers,
        }
    }

    fn change_destination(&mut self, destination: String) {
        self.destination = destination;
    }

    fn increase_price(&mut self) {
        self.price *= 1.2;
    }

    fn itinerary(&self) {
        println!(
            "{} -> {} with {} by {}",
            self.origin, self.destination, self.passengers, self.price
        );
    }
}

fn main() {
    let mut flight = Flight::new(String::from("Test"), String::from("Test 2"), 99.99, 50);
    flight.itinerary();
    flight.increase_price();
    flight.change_destination(String::from("Test 3"));
    flight.itinerary();
}
