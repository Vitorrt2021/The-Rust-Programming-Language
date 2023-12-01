#[derive(Debug)]
struct Rectangle {
    width: u64,
    height: u64,
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
}
