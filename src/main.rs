mod macros;
struct Rect {
    width: u32,
    height: u32,
}
struct Square {
    side: u32,
}

trait Shape {
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32;
}

impl Shape for Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}
impl Shape for Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }
    fn perimeter(&self) -> u32 {
        4 * self.side
    }
}

fn main() {
    println!("Hello, world!");
    let r = Rect {
        width: 20,
        height: 10,
    };
    let _c = Square {
        side: 10,
    };
    let (area, perimeter) = get_area_perimeter(&r);
    println!("Area of Rectangle: {}, and the perimeter: {}", area, perimeter);
    macros::macro_print();
}

fn get_area_perimeter(s: &impl Shape) -> (u32, u32) {
    (s.area(), s.perimeter())
}
