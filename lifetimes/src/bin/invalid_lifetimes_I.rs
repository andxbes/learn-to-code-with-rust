fn main() {
    let some_cities = {
        let cities = vec![
            String::from("London"),
            String::from("New York"),
            String::from("Barcelona"),
        ];

        // Вместо ссылки мы создаем новый вектор из этих элементов.
        // Теперь `some_cities` владеет своими данными.
        cities.into_iter().take(2).collect::<Vec<String>>()
    };
    println!("{:?}", some_cities);

    //Нельзя вернуть ссылку на данные которые будут уничтожены по окончанию их времени жизни в области видимости.
    let some_cities = {
        let cities = vec![
            String::from("London"),
            String::from("New York"),
            String::from("Barcelona"),
        ];
        // Invalid line
        // &cities[..2]
    };
}
