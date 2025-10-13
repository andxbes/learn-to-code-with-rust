mod inventory;
mod orders;
// old module
// mod orders {
//     pub const MANAGER: &str = "Oliver Orderson";
// }

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        inventory::MANAGER,
        orders::MANAGER,
        inventory::FLOOR_SPACE
    );

    inventory::talk_to_manager();

    let favorite_category = inventory::ProductCategory::Hammer;
    println!("My favorite category of item is {favorite_category:?}");

    let tail_lader = inventory::Item {
        name: String::from("Ladder-o-matic 2000"),
        category: favorite_category,
        quantity: 100,
    };

    println!("{:#?}", tail_lader);
}
