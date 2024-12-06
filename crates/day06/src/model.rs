#[derive(Debug, Clone, Copy)]
pub enum Position {
    Free(bool),
    Obstruction,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}
