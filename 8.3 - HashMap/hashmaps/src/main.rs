#![allow(unused_variables)]
fn main() {
    use std::collections::HashMap;

    // В стандартной библиотеке Rust есть структура данных HashMap<K,V> - словарь, где каждому
    // ключу можно сопоставить значение(и только одно). Для этого HashMap использует хеш-функцию.
    // Создать новый HashMap можно так:
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Ключи этого HashMap'а имеют тип String, а значения - i32. Все ключи и значения должны быть
    // одного и того же типа.
    // Создавать HashMap можно также и с помощью метода collect для вектора из кортежей, где каждый
    // кортеж состоит из двух элементов ключа и значения. Если ключи и значения находятся в разных
    // векторах, то можно воспользоваться методом zip для их объединения:
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // HashMap<_, _> позволяют компилятору самому выводить тип ключей и значений.

    // Типы, для которых реализован типаж Copy, будут копироваться в HashMap при вызове insert.
    // Остальные будут перемещаться и после insert обращаться к этим данным уже нельзя:
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // Следующий код вызове ошибку компиляции:
    // println!("{}", field_name);
    // println!("{}", field_value);

    // Получить значение по ключу можно с помощью метода get:
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    // get возвращает Option<&V>, поэтому нужен match, чтобы можно было извлечь само значение.
    match score {
        Some(&value) => println!("{}", value),
        None => (),
    }

    // По всему HashMap'у можно пройтись циклом for:
    for (key, value) in &scores {
        println!("{} - {}", key, value);
    }

    // Если в HashMap'е уже есть некая пара ключ-значение, то вставка ключа с новым значением
    // заменит старое значение этого ключа.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("scores: {:?}", scores);

    // Можно вставить значение для ключа, если такого ключа нет с помощью метода entry:
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // Обновление уже существующего значения:
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
