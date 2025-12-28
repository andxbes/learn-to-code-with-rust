mod inventory;
mod orders;

use inventory::{FLOOR_SPACE, Item, MANAGER as INVENTORY_MANAGER, ProductCategory};
//similar
//use crate::inventory::{FLOOR_SPACE, Item, MANAGER as INVENTORY_MANAGER, ProductCategory};
use orders::MANAGER as ORDERS_MANAGER;
fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        INVENTORY_MANAGER, ORDERS_MANAGER, FLOOR_SPACE
    );

    let favorite_category = ProductCategory::Hammer;
    println!("My favorite category of item is {favorite_category:?}");

    let tail_lader = Item::new(String::from("Ladder-o-matic 2000"), favorite_category, 100);

    println!("{:#?}", tail_lader);
}
