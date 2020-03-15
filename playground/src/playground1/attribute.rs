pub enum Attribute {
    Bool(bool),
    Text(String),
    Double(f64),
    Unsigned(u64),
    Signed(i64),
    Presence,
}
