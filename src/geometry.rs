/// Geometry
use std::ops::{Add, Mul};

/// Point

#[derive(Clone, Default, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn from_grid(g: Grid) -> Self {
        Point {
            x: g.x * 60,
            y: g.y * 60,
        }
    }

    /// Return nearest Point on grid.
    pub fn align_to_grid(&self) -> Point {
        Point::new((self.x + 30) / 60 * 60, (self.y + 30) / 60 * 60)
    }

    pub fn to_grid(&self) -> Grid {
        Grid::new((self.x + 30) / 60 * 60, (self.y + 30) / 60 * 60)
    }

    pub fn to_x_connecting_grid(&self) -> Grid {
        let (gx, gy) = ((self.x + 30) / 60, (self.y + 30) / 60);
        let (sx, _sy) = ((self.x - gx * 60).signum(), (self.y - gy * 60).signum());
        Grid::new(gx + sx, gy)
    }

    pub fn to_y_connecting_grid(&self) -> Grid {
        let (gx, gy) = ((self.x + 30) / 60, (self.y + 30) / 60);
        let (_sx, sy) = ((self.x - gx * 60).signum(), (self.y - gy * 60).signum());
        Grid::new(gx, gy + sy)
    }

    pub fn equal_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

impl PartialEq for Point {
    fn eq(&self, rhs: &Self) -> bool {
        (self.x == rhs.x) && (self.y == rhs.y)
    }
}

/// Implements the '+' operator for Point + Point
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// Implements the '*' (Inner Product) operator for Point * Point
impl Mul for Point {
    type Output = i32;

    fn mul(self, rhs: Point) -> i32 {
        self.x * rhs.x + self.y * rhs.y
    }
}

/// Grid

#[derive(Clone, Default, Copy)]
pub struct Grid {
    pub x: i32,
    pub y: i32,
}

impl Grid {
    pub fn new(x: i32, y: i32) -> Self {
        Grid { x, y }
    }

    pub fn from_point(p: Point) -> Self {
        Grid {
            x: (p.x + 30) / 60,
            y: (p.y + 30) / 60,
        }
    }

    pub fn to_point(&self) -> Point {
        Point::new(self.x * 60, self.y * 60)
    }
}

/// Vector

#[derive(Clone, Default, Copy)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {}