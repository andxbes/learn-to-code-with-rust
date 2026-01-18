fn main() {
    let mut first_name = String::from("Sylvester");

    first_name.push(' ');

    let spice = String::from(" - ");
    let last_name = String::from("Stallone");

    let full_name = first_name + &spice + &last_name;
    println!("{full_name}");

    // Invalid
    // println!("{first_name}");
    // println!("{first_name}");
    // println!("{last_name}");
}
