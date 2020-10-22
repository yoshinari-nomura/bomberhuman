/// Geometry
use std::ops::{Add, Mul};

/// Point

pub const GS: i32 = 60;

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
            x: g.x * GS,
            y: g.y * GS,
        }
    }

    /// Return nearest Point on grid.
    pub fn align_to_grid(&self) -> Point {
        Point::new((self.x + (GS / 2)) / GS * GS, (self.y + (GS / 2)) / GS * GS)
    }

    pub fn to_grid(&self) -> Grid {
        Grid::new((self.x + (GS / 2)) / GS * GS, (self.y + (GS / 2)) / GS * GS)
    }

    pub fn to_x_connecting_grid(&self) -> Grid {
        let (gx, gy) = ((self.x + (GS / 2)) / GS, (self.y + (GS / 2)) / GS);
        let (sx, _sy) = ((self.x - gx * GS).signum(), (self.y - gy * GS).signum());
        Grid::new(gx + sx, gy)
    }

    pub fn to_y_connecting_grid(&self) -> Grid {
        let (gx, gy) = ((self.x + (GS / 2)) / GS, (self.y + (GS / 2)) / GS);
        let (_sx, sy) = ((self.x - gx * GS).signum(), (self.y - gy * GS).signum());
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
            x: (p.x + (GS / 2)) / GS,
            y: (p.y + (GS / 2)) / GS,
        }
    }

    pub fn to_point(&self) -> Point {
        Point::new(self.x * GS, self.y * GS)
    }
}

/// Vector

#[derive(Clone, Default, Copy)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {}
