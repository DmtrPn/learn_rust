struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}
// Определение метода для доступного для определенного типа
#[allow(dead_code)]
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointMix<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> PointMix<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointMix<X2, Y2>) -> PointMix<X1, Y2> {
        PointMix {
            x: self.x,
            y: other.y,
        }
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    // let integer_and_float = Point { x: 5, y: 4.0 }; Error

     let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
    println!("p.y = {}", p.y());

    let p1 = PointMix { x: 5, y: 10.4 };
    let p2 = PointMix { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
