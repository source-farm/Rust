// Rust поддерживает структуры. Объявить структуру можно так:
struct User {
    // Поля(fields) структуры.
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
// Структуры можно объявлять и внутри функций.

fn main() {
    // Создание экземпляра(instance) User'а. Необходимо указывать значение каждого поля структуры,
    // но необъязательно в том порядке, в котором они объявлены.
    let x = User {
        name: String::from("John"),
        email: String::from("john@example.com"),
        active: false,
        sign_in_count: 0,
    };

    // Обращение к полям идёт через '.'.
    println!("{}, {}, {}, {}", x.name, x.email, x.sign_in_count, x.active);

    // Если создать экземпляр структуры как изменяемый, то можно менять значения полей:
    let mut y = User {
        name: String::from("James"),
        email: String::from(""),
        active: true,
        sign_in_count: 13,
    };
    y.email = String::from("james@example.com");
    println!("y.email: {}", y.email);

    let z = build_user(String::from("Jack"), String::from("jack@example.com"));
    println!("z.name: {}", z.name);

    // Можно более удобного создания экземпляров структур на основе уже существующих экземпляров
    // есть так называемый синтаксим обновления структур(struct update syntax):
    let u = User {
        name: String::from("Jane"),
        email: String::from("jane@example.com"),
        // Синтаксис '..' означает взять значения для явно не указанных полей из полей экземпляра в
        // переменной y.
        ..y
    };
    println!("u.sign_in_count: {}", u.sign_in_count);

    // Можно также определять структуры, которые не имеют названий полей. По поведению они очень
    // похожи на кортежи:

    // Структура из трёх полей типа i32(тип всех полей необъязательно должен совпадать).
    struct Point(i32, i32, i32);
    let origin = Point(0, 0, 0);
    // Обращение к полям идёт так же как и обращение к элементам кортежа.
    println!("origin: {},{},{}", origin.0, origin.1, origin.2);
}

fn build_user(name: String, email: String) -> User {
    // Как мы знаем последнее выражение функции является возвращаем значением этой функции. Можно
    // использовать это свойство для создания новых экземпляров User'а:
    // User {
    //     email: email,
    //     name: name,
    //     active: false,
    //     sign_in_count: 0,
    // }

    // Если название поля структуры и название параметра функции совпадают, то можно воспользоваться
    // более коротким синтаксисом инициализации полей:
    User {
        email, // То же самое, что и 'email: email'.
        name,
        active: false,
        sign_in_count: 0,
    }
}
