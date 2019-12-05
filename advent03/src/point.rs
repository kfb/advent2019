#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32
}

impl Point {
    pub fn add(&self, other: &Point) -> Point {
        Point {x: self.x + other.x, y: self.y + other.y}
    }

    pub fn cross(&self, other: &Point) -> f32 {
        (self.x * other.y) - (self.y * other.x)
    }

    pub fn dot(&self, other: &Point) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }

    pub fn manhattan(&self, from: &Point) -> f32 {
        ((self.x - from.x).abs() + (self.y - from.y).abs())
    }

    pub fn new(x: f32, y: f32) -> Point {
        Point {x: x, y: y}
    }

    pub fn scale(&self, factor: f32) -> Point {
        Point {x: self.x * factor, y: self.y * factor}
    }

    pub fn subtract(&self, other: &Point) -> Point {
        Point {x: self.x - other.x, y: self.y - other.y}
    }
}
