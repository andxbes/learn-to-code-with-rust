const FLOOR_SPACE: i32 = 1000;
pub const MANAGER: &str = "Ivan Inventory";

#[derive(Debug)]
enum ProductCategory {
    Loader,
    Hammer,
}

#[derive(Debug)]
struct Item {
    name: String,
    category: ProductCategory,
    quantity: u32,
}

fn talk_to_manager() {
    println!("Hey, {MANAGER}, how`s your coffee?");
}
