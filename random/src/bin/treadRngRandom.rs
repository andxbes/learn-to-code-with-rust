use rand::random;

fn main() {
    let random_float: f64 = random();
    let random_2 = random::<u8>();
    print!("{} {}", random_float, random_2);
}
