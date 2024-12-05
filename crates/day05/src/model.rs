#[derive(Debug, Clone)]
pub struct Rule {
    pub lhs: usize,
    pub rhs: usize,
}

#[derive(Debug, Clone)]
pub struct PageUpdates {
    pub list: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct ManualUpdateData {
    pub rules: Vec<Rule>,
    pub updates: Vec<PageUpdates>,
}
