use std::borrow::Cow;

use crate::FeatureFlag;

#[derive(Debug)]
pub struct ComponentRegistry {
    pub components: Vec<RegisteredComponent>,
}

#[derive(Debug)]
pub struct RegisteredComponent {
    pub features: Vec<FeatureFlag>,

    pub qualified_name: Cow<'static, str>,
    pub simple_name: Cow<'static, str>,

    pub required_attributes: Vec<Attribute>,
    pub optional_attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub struct Attribute {
    pub required: bool,
    pub name: Cow<'static, str>,
    pub value: Cow<'static, str>,
    pub default_code: Option<Cow<'static, str>>,
    pub features: Vec<FeatureFlag>,
}

#[derive(Debug)]
pub struct ComponentFeatureView<'a> {
    qualified_name: &'a str,
    simple_name: &'a str,

    required_attributes: Vec<&'a str>,
    optional_attributes: Vec<&'a str>,
}

impl RegisteredComponent {
    pub fn get_feature_view(&self, feature: Option<FeatureFlag>) -> Option<ComponentFeatureView> {
        if let Some(feature) = feature {
            if self.features.is_empty() || self.features.contains(&feature) {
                let optional_attributes = self.optional_attributes.iter()
                    .filter(|a| a.features.is_empty() || a.features.contains(&feature))
                    .map(|a| a.name.as_ref()).collect();
                let required_attributes = self.required_attributes.iter()
                    .filter(|a| a.features.is_empty() || a.features.contains(&feature))
                    .map(|a| a.name.as_ref()).collect();

                Some(
                    ComponentFeatureView {
                        qualified_name: &self.qualified_name,
                        simple_name: &self.simple_name,
                        optional_attributes,
                        required_attributes,
                    }
                )
            } else {
                None
            }
        } else if self.features.is_empty() {
            Some(
                ComponentFeatureView {
                    qualified_name: &self.qualified_name,
                    simple_name: &self.simple_name,
                    optional_attributes: self.optional_attributes.iter().filter(|a| a.features.is_empty()).map(|a| a.name.as_ref()).collect(),
                    required_attributes: self.required_attributes.iter().filter(|a| a.features.is_empty()).map(|a| a.name.as_ref()).collect(),
                }
            )
        } else {
            None
        }
    }
}