fn main() {
    let mut cereals = [
        "Cookie Crisp",
        "Cinnamon Toast Crunch",
        "Frosted Flakes",
        "Cocoa Puffs",
        "Captain Crunch",
    ];

    let first_two = &cereals[..2];
    println!("{:?}", first_two);
    let mid_three = &mut cereals[2..5];
    println!("{:?}", mid_three);

    mid_three[2] = "Lucky Charms";

    println!("{:?}", cereals);

    let cookie_crisp = cereals[0];

    let cookie = &cookie_crisp[..6];

    println!("{:?}", cookie);

    let cocoa_puffs = cereals[3];

    let puffs = &cocoa_puffs[6..];

    println!("{:?}", puffs);
}
