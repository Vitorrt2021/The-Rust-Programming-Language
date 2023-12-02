#[derive(Debug)]
struct Rectangle {
    width: u64,
    height: u64,
}

impl Rectangle {
    fn area(&self) -> u64 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn square(size: u64) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area_square(size: u64) -> u64 {
        size * size
    }   
}

fn build_rectangle(width: u64, height: u64) -> Rectangle {
    Rectangle { width, height }
}

fn area(rectangle: &Rectangle) -> u64 {
    rectangle.width * rectangle.height
}

fn main() {
    let rect = build_rectangle(30, 50);
    let area = area(&rect);
    println!("The rectangle {:#?} area is {}", rect, area);
    println!("The rectangle {:#?} area is {}", rect, rect.area());
    if rect.width() {
        println!("The rectangle has a nonzero width; it is {}", rect.width);
    }
    let square = Rectangle::square(200);
    println!("The square {:#?} area is {}", square, square.area());
    println!("The square area {}", Rectangle::area_square(square.width));
}
