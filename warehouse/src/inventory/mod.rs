pub mod products;
pub use products::{Item, ProductCategory};
pub const FLOOR_SPACE: i32 = 1000;
pub const MANAGER: &str = "Ivan Inventory";

pub fn talk_to_manager() {
    println!("Hey, {MANAGER}, how`s your coffee?");
    println!("{:?}", ProductCategory::Loader);
}
