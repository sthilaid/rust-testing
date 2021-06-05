enum Maybe<T> {
    Some(T),
    None,
}

struct Point<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Point<T> {
    fn manhattanDistance(&self, p: Point<T>) -> T {
        (self.x - p.x).abs() + (self.y - p.y).abs() + (self.z - p.z).abs()
    }
}

pub fn run() {
    let x = Point { x: 1, y: 2, z: 3 };
}
