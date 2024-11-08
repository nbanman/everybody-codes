use std::{fmt::Display, ops::Mul};

use num_traits::{One, PrimInt, Unsigned, Zero};

pub trait Coordinate: Default+PrimInt+Display+Zero+One+Mul {}

impl<T> Coordinate for T
where 
    T: Default+PrimInt+Display+Zero+One+Mul,
    {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coord<T: Coordinate, const N: usize>([T; N]);

pub type Coord2 = Coord<i64, 2>;
pub type Coord3 = Coord<i64, 3>;

impl<T: Coordinate, const N: usize>  Coord<T, N> {
    pub fn x(&self) -> T { self.0[0] }
}

impl<T: Coordinate+Unsigned, const N: usize>  Coord<T, N> {
    pub fn get_index(&self, dimensions: &[usize]) -> Option<usize> {
        let mut usized = Vec::with_capacity(N);
        for n in self.0 {
            let as_usize = n.to_usize()?;
            usized.push(as_usize);
        }

        let mut multipliers = Vec::with_capacity(N);
        let mut acc = 1;
        multipliers.push(acc);
        for &dim in dimensions {
            acc *= dim;
            multipliers.push(acc);
        }

        let index = usized.into_iter()
            .zip(multipliers.into_iter())
            .map(|(xyz, multiplier)| xyz * multiplier)
            .sum();
        Some(index)
    }
}

impl<T: Coordinate> Coord<T, 2> {
    pub fn new2d(x: T, y: T) -> Self {
        let mut contents= [T::default(); 2];
        contents[0] = x;
        contents[1] = y;
        Self(contents)
    }

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