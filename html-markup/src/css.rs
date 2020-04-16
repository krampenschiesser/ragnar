use ragnar_lib::Attribute;

#[derive(Debug, Default)]
pub struct CssClass(String);

#[derive(Debug, Default)]
pub struct CssStyle(String);

impl Into<String> for CssClass {
    fn into(self) -> String {
        self.0
    }
}

impl Into<String> for CssStyle {
    fn into(self) -> String {
        self.0
    }
}

impl Into<Attribute> for CssClass {
    fn into(self) -> Attribute {
        self.0.into()
    }
}

impl Into<Attribute> for CssStyle {
    fn into(self) -> Attribute {
        self.0.into()
    }
}

impl From<&str> for CssClass {
    fn from(class: &str) -> Self {
        CssClass(class.into())
    }
}

impl From<&str> for CssStyle {
    fn from(style: &str) -> Self {
        CssStyle(style.into())
    }
}