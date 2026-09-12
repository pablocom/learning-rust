use std::f64::consts::PI;

pub trait Area {
    fn calculate(&self) -> f64;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point2D {
    pub x: i64,
    pub y: i64,
}

impl Point2D {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Point2D) -> f64 {
        (self - other).magnitude()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector2D {
    pub dx: i64,
    pub dy: i64,
}

impl Vector2D {
    pub fn new(dx: i64, dy: i64) -> Self {
        Self { dx, dy }
    }

    pub fn dot(&self, other: &Vector2D) -> i64 {
        self.dx * other.dx + self.dy * other.dy
    }

    pub fn magnitude(&self) -> f64 {
        let dx = self.dx as f64;
        let dy = self.dy as f64;
        (dx * dx + dy * dy).sqrt()
    }
}

impl std::ops::Sub for &Point2D {
    type Output = Vector2D;

    fn sub(self, other: &Point2D) -> Vector2D {
        Vector2D::new(self.x - other.x, self.y - other.y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub top_right_corner: Point2D,
    pub top_left_corner: Point2D,
    pub bottom_right_corner: Point2D,
    pub bottom_left_corner: Point2D,
}

impl Rectangle {
    pub fn new(
        top_left_corner: Point2D,
        top_right_corner: Point2D,
        bottom_right_corner: Point2D,
        bottom_left_corner: Point2D,
    ) -> Result<Self, &'static str> {
        let vec_top = &top_right_corner - &top_left_corner;
        let vec_bottom = &bottom_right_corner - &bottom_left_corner;
        let vec_left = &bottom_left_corner - &top_left_corner;

        if vec_top != vec_bottom {
            return Err("Points do not form a closed parallelogram.");
        }

        if vec_top.dot(&vec_left) != 0 {
            return Err("Corners are not orthogonal (90 degrees).");
        }

        let rect = Self {
            top_right_corner,
            top_left_corner,
            bottom_right_corner,
            bottom_left_corner,
        };

        if rect.calculate() == 0.0 {
            return Err("Rectangle must have a positive area.");
        }

        Ok(rect)
    }
}

impl Area for Rectangle {
    fn calculate(&self) -> f64 {
        let width = self.top_left_corner.distance_to(&self.top_right_corner);
        let height = self.top_left_corner.distance_to(&self.bottom_left_corner);
        width * height
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point2D,
    pub radius: f64,
}

impl Circle {
    pub fn new(center: Point2D, radius: f64) -> Result<Self, &'static str> {
        if radius <= 0.0 {
            return Err("Radius must be greater than zero.");
        }
        Ok(Self { center, radius })
    }
}

impl Area for Circle {
    fn calculate(&self) -> f64 {
        PI * self.radius * self.radius
    }
}
