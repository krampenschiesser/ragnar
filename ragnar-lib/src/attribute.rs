use std::borrow::Cow;

#[derive(Debug, PartialEq, Clone,serde::Serialize,serde::Deserialize)]
pub enum Attribute {
    Bool(bool),
    Text(Cow<'static, str>),
    FloatingPoint(f64),
    Unsigned(u64),
    Signed(i64),
    Presence,
}

impl From<char> for Attribute {
    fn from(c: char) -> Self {
        let mut str = String::with_capacity(1);
        str.push(c);
        Attribute::Text(Cow::Owned(str))
    }
}

impl From<String> for Attribute {
    fn from(c: String) -> Self {
        Attribute::Text(Cow::Owned(c))
    }
}
impl From<&'static str> for Attribute {
    fn from(c: &'static str) -> Self {
        Attribute::Text(Cow::Borrowed(c))
    }
}

impl From<Cow<'static, str>> for Attribute {
    fn from(c: Cow<'static, str>) -> Self {
        Attribute::Text(c)
    }
}

impl From<bool> for Attribute {
    fn from(b: bool) -> Self {
        Attribute::Bool(b)
    }
}

impl From<u32> for Attribute {
    fn from(value: u32) -> Self {
        Self::Unsigned(value as u64)
    }
}

impl From<u16> for Attribute {
    fn from(value: u16) -> Self {
        Self::Unsigned(value as u64)
    }
}

impl From<usize> for Attribute {
    fn from(value: usize) -> Self {
        Self::Unsigned(value as u64)
    }
}

impl From<u64> for Attribute {
    fn from(value: u64) -> Self {
        Self::Unsigned(value)
    }
}

impl From<i32> for Attribute {
    fn from(value: i32) -> Self {
        Self::Signed(value as i64)
    }
}

impl From<i16> for Attribute {
    fn from(value: i16) -> Self {
        Self::Signed(value as i64)
    }
}

impl From<isize> for Attribute {
    fn from(value: isize) -> Self {
        Self::Signed(value as i64)
    }
}

impl From<i64> for Attribute {
    fn from(value: i64) -> Self {
        Self::Signed(value)
    }
}

impl From<f32> for Attribute {
    fn from(value: f32) -> Self {
        Self::FloatingPoint(value as f64)
    }
}

impl From<f64> for Attribute {
    fn from(value: f64) -> Self {
        Self::FloatingPoint(value)
    }
}