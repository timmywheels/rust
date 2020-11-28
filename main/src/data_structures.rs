use std::fmt::Display;
use core::fmt;

struct Point {
    x: f64,
    y: f64
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.start, self.end)
    }
}

struct Line {
    start: Point,
    end: Point
}

fn structures() {
    let p = Point{ x:5.5, y:10.1 };
    println!("point: {}", p);
    let p2 = Point { x: 5.0, y: 10.0 };
    let line = Line{ start: p, end: p2 };
    println!("line: {}", line);
}

pub fn data_structures() {
    structures();
}