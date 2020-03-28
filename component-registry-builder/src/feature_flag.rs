#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct FeatureFlag(pub String);

impl<'a> From<&'a str> for FeatureFlag {
    fn from(val: &'a str) -> Self {
        Self(val.into())
    }
}
