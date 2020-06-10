use std::cmp::Ordering;
use std::convert::TryFrom;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

pub use Quadrant::*;
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Quadrant {
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

// use enum instead with East, West, etc. variants?
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Offset {
    pub dx: i64,
    pub dy: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Level(pub u8);

impl From<(i64, i64)> for Position {
    fn from(t: (i64, i64)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl Position {
    pub const ORIGIN: Self = Self::new(0, 0);

    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn quadrant(self) -> Quadrant {
        match (self.x < 0, self.y < 0) {
            (true, true) => Quadrant::NorthWest,
            (false, true) => Quadrant::NorthEast,
            (true, false) => Quadrant::SouthWest,
            (false, false) => Quadrant::SouthEast,
        }
    }

    pub fn relative_to(self, other: Self) -> Self {
        self.offset((-other.x, -other.y))
    }

    // use `Add` trait?
    pub fn offset(self, offset: impl Into<Offset>) -> Self {
        let offset = offset.into();
        Self::new(self.x + offset.dx, self.y + offset.dy)
    }

    pub fn in_bounds(self, level: Level) -> bool {
        let bounds = level.coord_range();
        bounds.contains(&self.x) && bounds.contains(&self.y)
    }
}

impl From<(i64, i64)> for Offset {
    fn from(t: (i64, i64)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl Offset {
    pub const fn new(dx: i64, dy: i64) -> Self {
        Self { dx, dy }
    }
}

impl PartialEq<u8> for Level {
    fn eq(&self, n: &u8) -> bool {
        self.0 == *n
    }
}

impl PartialOrd<u8> for Level {
    fn partial_cmp(&self, n: &u8) -> Option<Ordering> {
        Some(self.0.cmp(n))
    }
}

//impl Ord<u8> for Level {
//fn cmp(&self, n: &u8) -> Ordering {
//self.0.cmp(n)
//}
//}

impl Add for Level {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Level(self.0 + other.0)
    }
}

impl Add<u8> for Level {
    type Output = Self;

    fn add(self, n: u8) -> Self {
        Level(self.0 + n)
    }
}

impl AddAssign for Level {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl AddAssign<u8> for Level {
    fn add_assign(&mut self, n: u8) {
        self.0 += n;
    }
}

impl Sub for Level {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Level(self.0 - other.0)
    }
}

impl Sub<u8> for Level {
    type Output = Self;

    fn sub(self, n: u8) -> Self {
        Level(self.0 - n)
    }
}

impl SubAssign for Level {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl SubAssign<u8> for Level {
    fn sub_assign(&mut self, n: u8) {
        self.0 -= n;
    }
}

impl Level {
    pub const MAX_LEVEL: Self = Self(63);
    pub const LEAF_LEVEL: Self = Self(0);

    pub const fn side_len(self) -> u64 {
        1 << self.0
    }

    pub fn quadrant_center(self, quadrant: Quadrant) -> Position {
        let delta = i64::try_from(self.side_len() / 4).unwrap();
        match quadrant {
            NorthWest => (-delta, -delta).into(),
            NorthEast => (delta, -delta).into(),
            SouthWest => (-delta, delta).into(),
            SouthEast => (delta, delta).into(),
        }
    }

    pub const fn min_coord(self) -> i64 {
        -(1 << (self.0 - 1))
    }

    pub const fn max_coord(self) -> i64 {
        (1 << (self.0 - 1)) - 1
    }

    pub const fn coord_range(self) -> std::ops::Range<i64> {
        self.min_coord()..self.max_coord()
    }

    pub fn min_pos(self) -> Position {
        let min = Self::min_coord(self);
        (min, min).into()
    }

    pub fn max_pos(self) -> Position {
        let max = Self::max_coord(self);
        (max, max).into()
    }
}
