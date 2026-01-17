use warehouse::{FLOOR_SPACE, INVENTORY_MANAGER, ORDERS_MANAGER};

fn main() {
    println!(
        "Our manager are {} and {}. We have {} square feet od spice",
        INVENTORY_MANAGER, ORDERS_MANAGER, FLOOR_SPACE
    );
}
