#[derive(Debug)]

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Self {
        Self { x, y }
    }

    fn mixup<X, Y>(self, point: Point<X, Y>) -> Point<T, Y> {
        Point {
            x: self.x,
            y: point.y,
        }
    }
}

impl Point<f64, f64> {
    fn calculate_distance(&self) -> f64 {
        4.0
    }
}

fn main() {
    let point_a = Point::new(4, 5.5);
    let point_b = Point::new(6.3, 5.3);

    point_b.calculate_distance();

    let point_c = point_a.mixup(point_b);

    println!("{:?}", point_c);
}
