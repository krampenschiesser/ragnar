#[derive(Debug, Eq, PartialEq, Hash, Clone,serde::Serialize,serde::Deserialize)]
pub struct FeatureFlag(pub String);

impl<'a> From<&'a str> for FeatureFlag {
    fn from(val: &'a str) -> Self {
        Self(val.into())
    }
}
