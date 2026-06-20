fn double_the_length<T>(collection: &Vec<T>) -> usize {
    collection.len() * 2
}

fn last_two<T>(collection: &[T]) -> &[T] {
    let end = collection.len() - 2;

    &collection[end..]
}

fn first_five<'a>(text: &'a str, announcement: &str) -> &'a str {
    println!("{announcement}");
    &text[..5]
}

fn find_string_that_has_content<'a>(first: &'a str, secont: &'a str, target: &'a str) -> &'a str {
    if (first.contains(target)) {
        first
    } else {
        secont
    }
}

fn main() {
    println!("{}", double_the_length(&vec![1, 2, 3]));
    println!("{}", double_the_length(&vec![1, 2, 3, 4]));

    println!("{:?}", last_two(&vec![1, 2, 3]));
    println!("{:?}", last_two(&vec![1, 2, 3, 4, 5, 6]));

    println!("{}", first_five("refrigerator", "Hello"));

    println!(
        "{}",
        find_string_that_has_content("programming", "dining", "gram")
    );
}
