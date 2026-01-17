use fake::{Fake, Faker};
//use warehouse::{FLOOR_SPACE, INVENTORY_MANAGER, Item, ORDERS_MANAGER, ProductCategory};
use warehouse::*;

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        INVENTORY_MANAGER, ORDERS_MANAGER, FLOOR_SPACE
    );

    let favorite_category = ProductCategory::Hammer;
    println!("My favorite category of item is {favorite_category:?}");

    let tail_lader = Item::new(String::from("Ladder-o-matic 2000"), favorite_category, 100);

    println!("{:#?}", tail_lader);

    let random_category: ProductCategory = Faker.fake();

    println!("{:#?}", random_category);
}
