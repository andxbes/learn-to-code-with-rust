use fake::{Fake, Faker};

use warehouse::ProductCategory;
fn main() {
    let random_category: ProductCategory = Faker.fake();

    println!("{:#?}", random_category);
}
