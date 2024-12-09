#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiskContent {
    pub content_type: ContentType,
    pub length: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    Free,
    File(usize),
}
