fn main() {
    let rectangle = Rectangle {
        width: 3840,
        height: 2160,
    };

    println!("{:#?}", rectangle);
    println!(
        "N of pixels: {}",
        rectangle.area()
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
