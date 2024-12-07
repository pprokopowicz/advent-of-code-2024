#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub test_value: usize,
    pub numbers: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Addition,
    Multiplication,
    Concatenation,
}
