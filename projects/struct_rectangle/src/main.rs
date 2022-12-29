#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 10;
    let height1 = 10;
    println!(
        "The area of the rectangle is {} square pixels",
        area1(width1, height1)
    );

    let rect1 = (50, 50);
    println!(
        "The area of the rectangle is {} square pixels",
        area2(rect1)
    );

    let rect1 = Rectangle {
        width: 100,
        height: 200,
    };
    println!(
        "The area of the rectangle is {} square pixels",
        area3(&rect1)
    );

    let rect1 = Rectangle {
        width: 321,
        height: 543,
    };
    println!("rect1 is {:#?}", rect1);

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
