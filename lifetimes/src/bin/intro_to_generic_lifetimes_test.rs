fn select_first_two_elements(items: &[String]) -> &[String] {
    &items[..2]
}

fn move_first_two_elements(items: Vec<String>) -> Vec<String> {
    // Здесь items — владелец вектора.
    // После into_iter() элементы перемещаются из вектора.
    items.into_iter().take(2).collect()
}

fn drain_first_two_elements(items: &mut Vec<String>) -> Vec<String> {
    // drain(0..2) удаляет элементы из вектора и возвращает их
    // Вектор по адресу items остается живым, но становится короче
    items.drain(0..2).collect()
}

fn main() {
    let mut cities = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona"),
        String::from("Tokyo"),
    ];

    // Вариант 1: Заимствование (ссылка)
    let two_cities = select_first_two_elements(&cities);
    println!("1. Просто посмотрели: {two_cities:?}");
    println!("Оригинал все еще доступен: {cities:?}");

    // Вариант 3: Извлечение (Drain)
    // Мы передаем изменяемую ссылку. Вектор 'cities' не исчезает,
    // но он ТЕРЯЕТ владение первыми двумя элементами.
    let drained = drain_first_two_elements(&mut cities);

    println!("3. Извлеченные элементы (владеем ими): {drained:?}");
    println!("Оригинал остался у нас, но похудел: {cities:?}");

    // Вариант 2: Полное перемещение (Move)
    // Мы отдаем ВЕСЬ вектор.
    let moved = move_first_two_elements(cities);
    println!("2. Забрали остатки: {moved:?}");

    // println!("{cities:?}"); // ОШИБКА: cities больше не существует
}
