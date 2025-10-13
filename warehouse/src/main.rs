mod inventory;
mod orders;
// old module
// mod orders {
//     pub const MANAGER: &str = "Oliver Orderson";
// }

fn main() {
    println!("The manager of our inventory is {}", inventory::MANAGER);
    println!("The manager of our inventory is {}", orders::MANAGER);
}
