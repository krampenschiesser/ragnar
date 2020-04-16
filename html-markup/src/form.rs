use std::ops::Deref;
use ragnar_lib::Attribute;

#[derive(Debug,Clone,Eq,PartialEq)]
pub struct FormId(String);


impl Deref for FormId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<String> for FormId {
    fn into(self) -> String {
        self.0
    }
}


impl Into<Attribute> for FormId {
    fn into(self) -> Attribute {
        self.0.into()
    }
}