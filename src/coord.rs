use std::fmt::Display;

use num_traits::{CheckedAdd, CheckedSub, One, Zero};

pub trait Coordinate: Copy+Default+CheckedAdd+CheckedSub+Display+Zero+One+Ord {}

impl Coordinate for usize {}
impl Coordinate for u128 {}
impl Coordinate for u64 {}
impl Coordinate for u32 {}
impl Coordinate for u16 {}
impl Coordinate for u8 {}
impl Coordinate for isize {}
impl Coordinate for i128 {}
impl Coordinate for i64 {}
impl Coordinate for i32 {}
impl Coordinate for i16 {}
impl Coordinate for i8 {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coord<T: Coordinate, const N: usize>([T; N]);

pub type Coord2 = Coord<i64, 2>;
pub type Coord3 = Coord<i64, 3>;

impl<T: Coordinate> Coord<T, 2> {
    pub fn new2d(x: T, y: T) -> Self {
        let mut contents= [T::default(); 2];
        contents[0] = x;
        contents[1] = y;
        Self(contents)
    }

    pub fn x(&self) -> T { self.0[0] }
    pub fn y(&self) -> T { self.0[1] }

    pub fn origin() -> Self {
        Self([T::default(); 2])
    }

    pub fn adjacent(&self, diagonals: bool) -> Vec<Self> {
        let capacity = if diagonals { 8 } else { 4 };

        let mut neighbors = Vec::with_capacity(capacity);
        // north
        if let Some(y) = self.y().checked_sub(&T::one()) {
            neighbors.push(Self::new2d(self.x(), y));
        }
        // northeast
        if diagonals {
            if let Some(y) = self.y().checked_sub(&T::one()) {
                if let Some(x) = self.x().checked_add(&T::one()) {
                    neighbors.push(Self::new2d(x, y));
                }
            }
        }
        // east
        if let Some(x) = self.x().checked_add(&T::one()) {
            neighbors.push(Self::new2d(x, self.y()));
        }
        // southeast
        if diagonals {
            if let Some(y) = self.y().checked_add(&T::one()) {
                if let Some(x) = self.x().checked_add(&T::one()) {
                    neighbors.push(Self::new2d(x, y));
                }
            }
        }
        // south
        if let Some(y) = self.y().checked_add(&T::one()) {
            neighbors.push(Self::new2d(self.x(), y));
        }
        // southwest
        if diagonals {
            if let Some(y) = self.y().checked_add(&T::one()) {
                if let Some(x) = self.x().checked_sub(&T::one()) {
                    neighbors.push(Self::new2d(x, y));
                }
            }
        }
        // west
        if let Some(x) = self.x().checked_sub(&T::one()) {
            neighbors.push(Self::new2d(x, self.y()));
        }
        // northwest
        if diagonals {
            if let Some(y) = self.y().checked_sub(&T::one()) {
                if let Some(x) = self.x().checked_sub(&T::one()) {
                    neighbors.push(Self::new2d(x, y));
                }
            }
        }
        neighbors
    }

}

impl<T: Coordinate> Coord<T, 3> {
    pub fn new3d(x: T, y: T, z: T) -> Self {
        let mut contents= [T::default(); 3];
        contents[0] = x;
        contents[1] = y;
        contents[2] = z;
        Self(contents)
    }

    pub fn x(&self) -> T { self.0[0] }
    pub fn y(&self) -> T { self.0[1] }
    pub fn z(&self) -> T { self.0[1] }

    pub fn origin() -> Self {
        Self([T::default(); 3])
    }
}

impl<T: Coordinate> Display for Coord<T, 2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x(), self.y())
    }
}

impl<T: Coordinate> PartialOrd for Coord<T, 2> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.y().cmp(&other.y()).then_with(|| self.x().cmp(&other.x())))
    }
}

impl<T: Coordinate> Ord for Coord<T, 2> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y().cmp(&other.y()).then_with(|| self.x().cmp(&other.x()))
    }
}