#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OrderRecordError {
    NotFound,
    Unexpected,
}
