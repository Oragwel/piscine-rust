use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    // Method to calculate the distance between two points
    pub fn distance(&self, other: Point) -> f64 {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    // Associated function to create a new Circle
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Self {
            center: Point(x, y),
            radius,
        }
    }

    // Method to calculate the diameter of the circle
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

    // Method to calculate the area of the circle
    pub fn area(&self) -> f64 {
        PI * self.radius.powi(2)
    }

    // Method to check if two circles intersect
    pub fn intersect(&self, other: Circle) -> bool {
        self.center.distance(other.center) < self.radius + other.radius
    }
}
