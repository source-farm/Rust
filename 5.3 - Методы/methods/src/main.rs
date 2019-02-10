#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Определение методов для Rectangle должно идти внутри блока impl для Rectangle.
impl Rectangle {
    // Rust позволяет создавать для структур(также для перечислений и типажей) методы. По поведению
    // они похожи на функцию, но связаны только с какой-то структурой. Первым параметром метода
    // всегда является self - экземпляр структуры, для которой и был вызван этот метод. &self
    // означает, что идёт неизменяемое заимствование(immutable borrow). Методы, как и обычные
    // функции, могут получать владение(self), изменяемые ссылки(&mut self) или неизменяемые
    // ссылки(&self).
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Метод, которому можно передвать один аргумент.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Внутри блока impl можно также определять и так называемые ассоциированные функции(associated
    // functions) - это функции, у которых нет параметра self(это именно функции, а не методы).
    // Обычно такие функции используют для создания новых экземпляров. Например, для создания
    // квадрата:
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    // Для одной структуры можно использовать несколько impl блоков. Например, можно было бы
    // определить методы area и can_hold каждый в своём блоке impl. Делать это для структуры
    // Rectangle не имеет смысла, но несколько impl блоков могут быть полезны при работа с дженериками
    // или типажами.
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 area: {}", rect1.area());

    let rect2 = Rectangle {
        width: 20,
        height: 15,
    };
    println!("can hold: {}", rect1.can_hold(&rect2));

    // Вызов ассоциированной функции.
    let sq = Rectangle::square(11);
    println!("{:?}", sq);
}
