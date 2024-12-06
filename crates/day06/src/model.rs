#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Free(bool),
    Obstruction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}
