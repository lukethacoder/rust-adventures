#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

// similar to interfaces
trait HasArea {
    fn area(&self) -> f32;
}

// enum GoodNoGood<T> {
//     Good(T),
//     NoGood
// }

enum Shape {
    Circle(f32),
    Square(f32),
    Rectangle(f32, f32),
}

impl HasArea for Shape {
    fn area(&self) -> f32 {
        match self {
            Shape::Circle(r) => 3.14159 * r * r,
            Shape::Square(l) => l * l,
            Shape::Rectangle(l, w) => l * w,
        }
    }
}

impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    fn divide(self, v: f32) -> Result<Point, String> {
        if v == 0.0 {
            Err("Divide by Zero".to_string())
        } else {
            Ok(Point {
                x: self.x / v,
                y: self.y / v,
            })
        }
    }

    fn divideStatic(point: Point, v: f32) -> Option<Point> {
        if v == 0.0 {
            None
        } else {
            Some(Point {
                x: point.x / v,
                y: point.y / v,
            })
        }
    }
}

fn main() {
    let shape = Shape::Circle(3.0);

    // List<T> - acts like an array, sor of.
    let mut shapes = Vec::new();
    shapes.push(shape);

    println!("Shape: {:?}", shapes[0].area());

    // let point = Point::new(2.0, 4.0);
    // // let divided = Point::divideStatic(point, 2.0);
    // // passes itself in as self as first param.
    // let divided_maybe = point.divide(2.0);

    // let divided = match divided_maybe {
    //     Err(msg) => panic!(msg),
    //     Ok(point) => point,
    // };

    // println!("{:?}", point);
    // println!("{:?}", divided);
}
