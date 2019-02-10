#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("area: {}", area(&rect1));
    // Отладочный вывод.
    println!("rect: {:?}", rect1);
    // Более приятный отладочный вывод.
    println!("rect: {:#?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
