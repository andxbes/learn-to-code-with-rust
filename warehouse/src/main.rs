mod inventory;
mod orders;

use crate::inventory::{FLOOR_SPACE, talk_to_manager};
use inventory::products::{Item, ProductCategory};

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        // crate::inventory::MANAGER,
        inventory::MANAGER,
        orders::MANAGER,
        FLOOR_SPACE
    );

    let favorite_category = ProductCategory::Hammer;
    println!("My favorite category of item is {favorite_category:?}");

    let tail_lader = Item::new(String::from("Ladder-o-matic 2000"), favorite_category, 100);

    println!("{:#?}", tail_lader);
}
