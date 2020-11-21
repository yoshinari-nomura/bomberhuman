//! Geometry

use std::cmp::max;
use std::ops::{Add, AddAssign, Mul, Sub};

/// Cardinal Direction  N/W/S/E
pub enum Direction {
    N,
    W,
    S,
    E,
}

/// Grid size: Width and height of each Grid
///
/// In this game, almost all game characters (actors) are to be
/// located on the centers of grids like (n * GS, m * GS).
///
/// This also means that every character has the size of GS x GS.
pub const GS: i32 = 60;

/// Point: (x, y) style location, also known as Vector or Grid
///
/// Since almost all game characters (actors) are to be located on the
/// centers of grids, Point has some alignment functions and macros
/// such as `align_to_grid`, `grd!`, `vector_toward_grid`
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate bomberhuman; fn main() {
/// use bomberhuman::geometry::*;
///
/// let pnt = Point::new(0, 0);
/// assert_eq!(pnt, pnt!(0, 0));
/// assert_eq!(pnt.is_zero(), true);
///
/// let grd = grd!(115, 160);
/// assert_eq!(pnt!(120, 180), grd); // grid size (GS) is 60
/// assert_eq!(pnt!(115, 160).align_to_grid(), grd!(120, 180));
/// # }
/// ```
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Short version of Point::new(x, y)
#[allow(unused_macros)]
#[macro_export]
macro_rules! pnt {
    ($x:expr, $y:expr) => {{
        let x: i32 = $x;
        let y: i32 = $y;
        $crate::geometry::Point::new(x, y)
    }};
}

/// Short version of Point::new(x, y).align_to_grid();
#[allow(unused_macros)]
#[macro_export]
macro_rules! grd {
    ($x:expr, $y:expr) => {{
        let x: i32 = $x;
        let y: i32 = $y;
        pnt!((x + GS / 2) / GS * GS, (y + GS / 2) / GS * GS)
    }};
}

impl Point {
    /// Construct Point with x, y
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    /// Create the nearest grid point.
    pub fn align_to_grid(&self) -> Grid {
        grd!(self.x, self.y)
    }

    /// Predicate the point equals to (0, 0)
    pub fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    /// Length of Vector
    pub fn length(&self) -> i32 {
        max(self.x.abs(), self.y.abs())
    }

    /// Abstract direction N/W/S/E
    pub fn cardinal_direction(&self) -> Option<Direction> {
        if self.x < 0 {
            Some(Direction::W)
        } else if self.x > 0 {
            Some(Direction::E)
        } else if self.y < 0 {
            Some(Direction::N)
        } else if self.y > 0 {
            Some(Direction::S)
        } else {
            None
        }
    }

    /// Clip the norm of Vector into `length`
    pub fn clip_length(&self, length: i32) -> Vector {
        let (x, y) = self.to_tuple();
        pnt!(
            if x.abs() > length {
                x.signum() * length
            } else {
                x
            },
            if y.abs() > length {
                y.signum() * length
            } else {
                y
            }
        )
    }

    /// Convert the point into tuple (x, y)
    pub fn to_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    /// Calculate Vector toward the nearest grid point
    pub fn vector_toward_grid(&self) -> Vector {
        self.align_to_grid() - *self
    }

    /// Predicate the point collides with `pnt`
    pub fn collides_with(&self, pnt: Point) -> bool {
        (self.x - pnt.x).abs() < GS && (self.y - pnt.y).abs() < GS
    }

    /// Adjust the vector to go through the nearest grid
    ///
    /// # Algorism
    ///
    /// 1. Let `v = (dx, dy)` be the original vector and (gx, gy) be the
    ///    vector from its current position (x, y) to the nearest grid.
    /// 2. Let θ be the angle formed by (dx, dy) and (gx, gy)
    /// 3. if θ is within ±90 degrees (inner product is 0 or greater) → (gx, gy).
    /// 4. else, use (dx, dy)
    ///
    pub fn adjust_vector_to_grid(&self, v: Vector) -> Vector {
        let gv = self.vector_toward_grid();
        let speed = v.length();

        // It's not moving or already on the grid
        if v.is_zero() || gv.is_zero() {
            return v;
        }

        if gv * v >= 0 {
            // Within 90 degree angle to the center of grid,
            // make it go to the center of the grid.
            // Clip the size of the vector to less than the original speed.
            gv.clip_length(speed)
        } else {
            // It's moving away from the grid.
            v
        }
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

/// Implements the '+=' operator for Point += Point
impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        *self = Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

/// Implements the '-' operator for Point + Point
impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
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
pub type Grid = Point;

/// Vector
pub type Vector = Point;
