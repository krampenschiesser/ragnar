use crate::FeatureFlag;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegisteredComponent {
    pub features: Vec<FeatureFlag>,

    pub qualified_name: String,
    pub simple_name: String,
    pub render_type: RenderType,

    pub required_attributes: Vec<Attribute>,
    pub optional_attributes: Vec<Attribute>,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum RenderType {
    App,
    Local,
    Native,
    None,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Attribute {
    pub required: bool,
    pub name_view: String,
    pub name_code: String,
    pub value: String,
    pub sub_component: Option<String>,
    pub default_code: Option<String>,
    pub features: Vec<FeatureFlag>,
}

#[derive(Debug)]
pub struct ComponentFeatureView<'a> {
    pub qualified_name: &'a str,
    pub simple_name: &'a str,
    pub render_type: &'a RenderType,

    pub required_attributes: Vec<&'a Attribute>,
    pub optional_attributes: Vec<&'a Attribute>,
    pub sub_components: Vec<String>,
}

fn any_match(a: &[FeatureFlag], b: &[FeatureFlag]) -> bool {
    a.iter().any(|f| b.contains(f))
}

impl RegisteredComponent {
    pub fn get_feature_view(&self, expected_features: &Vec<FeatureFlag>) -> Option<ComponentFeatureView> {
        if !expected_features.is_empty() {
            if self.features.is_empty() || any_match(&self.features, expected_features) {
                let optional_attributes = self.optional_attributes.iter()
                    .filter(|a| a.features.is_empty() || any_match(&a.features, expected_features))
                    .collect();
                let required_attributes = self.required_attributes.iter()
                    .filter(|a| a.features.is_empty() || any_match(&a.features, expected_features))
                    .collect();

                let sub_components = self.optional_attributes.iter()
                    .filter(|a| a.features.is_empty() || any_match(&a.features, expected_features))
                    .filter_map(|a| a.sub_component.clone())
                    .collect();
                Some(
                    ComponentFeatureView {
                        qualified_name: &self.qualified_name,
                        simple_name: &self.simple_name,
                        optional_attributes,
                        required_attributes,
                        sub_components,
                        render_type: &self.render_type,
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
                    optional_attributes: self.optional_attributes.iter().filter(|a| a.features.is_empty()).collect(),
                    required_attributes: self.required_attributes.iter().filter(|a| a.features.is_empty()).collect(),
                    sub_components: self.optional_attributes.iter().filter_map(|a| a.sub_component.clone()).collect(),
                    render_type: &self.render_type,
                }
            )
        } else {
            None
        }
    }
}

impl<'a> ComponentFeatureView<'a> {
    pub fn contains_attribute(&self, name: &str) -> bool {
        self.required_attributes.iter().any(|a|a.name_view==name)
            || self.optional_attributes.iter().any(|a|a.name_view==name)
    }
}