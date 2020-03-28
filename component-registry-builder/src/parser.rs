use pest::Parser;
use crate::FeatureFlag;
use pest::iterators::{Pair, Pairs};
use crate::parse_error::ParseError;
use crate::component_registry::{RegisteredComponent, Attribute};
use std::process::id;
use pest::error::ErrorVariant::ParsingError;
use std::borrow::Cow;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct RustSourceParser;

pub struct DeclaredModule {
    pub name: String,
    pub feature_flag: Option<FeatureFlag>,
}

pub struct ParseResult {
    pub modules: Vec<DeclaredModule>,
    pub components: Vec<RegisteredComponent>,
}

impl RustSourceParser {
    pub fn parse_data(data: &str) -> Result<ParseResult, ParseError> {
        let mut parse_result = ParseResult {
            modules: Vec::new(),
            components: Vec::new(),
        };

        let parsed: pest::iterators::Pairs<Rule> = RustSourceParser::parse(Rule::File, data)?;
        for p in parsed.into_iter().next().ok_or(ParseError::NoFileContent)?.into_inner() {
            let rule = p.as_rule();
            match rule {
                Rule::Module => {
                    let module = parse_module(p.into_inner())?;
                    parse_result.modules.push(module);
                }
                Rule::StructDef => {
                    let component = parse_struct(p.into_inner())?;
                    parse_result.components.push(component);
                }
                _ => {}
            }
        };
        Ok(parse_result)
    }
}

fn parse_module(pairs: Pairs<Rule>) -> Result<DeclaredModule, ParseError> {
    let mut feature_name: Option<&str> = None;
    let mut module_name: Option<&str> = None;
    for p in pairs {
        match p.as_rule() {
            Rule::FeatureGate => {
                feature_name = parse_feature_gate(p.into_inner())?;
            }
            Rule::Identifier => {
                // let l1 = format!("{:?}",p);
                // let inner = p.into_inner();
                // let l2 = format!("{:?}",&inner);
                // let inner = inner.into_iter().next().ok_or(ParseError::NoIdentifier)?;
                module_name = Some(p.as_str());
            }
            _ => Err(ParseError::InvalidModuleContent(p.as_str().into()))?
        }
    }

    if let Some(module_name) = module_name {
        let mut module = DeclaredModule {
            name: module_name.into(),
            feature_flag: None,
        };
        if let Some(feature_name) = feature_name {
            module.feature_flag = Some(feature_name.into());
        }
        Ok(module)
    } else {
        Err(ParseError::CouldNotParseModule)
    }
}

fn parse_struct(pairs: Pairs<Rule>) -> Result<RegisteredComponent, ParseError> {
    let mut required_attributes = Vec::new();
    let mut optional_attributes = Vec::new();
    let mut identifier: Option<&str> = None;
    let mut feature_flag = None;

    for p in pairs {
        match p.as_rule() {
            Rule::FeatureGate => {
                feature_flag = parse_feature_gate(p.into_inner())?.map(|s| FeatureFlag::from(s));
            }
            Rule::Identifier => {
                identifier = Some(p.as_str());
            }
            Rule::Attribute => {
                let attr: Attribute = parse_attribute(p.into_inner())?;
                if attr.required {
                    required_attributes.push(attr);
                } else {
                    optional_attributes.push(attr);
                }
            }
            _ => Err(ParseError::InvalidStructContent(p.as_str().into()))?
        }
    }

    if let Some(identifier) = identifier {
        let mut features = Vec::new();
        if let Some(f) = feature_flag {
            features.push(f)
        }
        Ok(RegisteredComponent {
            qualified_name: "".into(),
            simple_name: Cow::Owned(identifier.into()),
            required_attributes,
            optional_attributes,
            features,
        })
    } else {
        Err(ParseError::CouldNotFindStructIdentifier)
    }
}

fn parse_attribute(pairs: Pairs<Rule>) -> Result<Attribute, ParseError> {
    let mut required = false;
    let mut default_code: Option<String> = None;
    let mut feature_flag = None;
    let mut attribute_value = None;
    let mut attribute_identifier = None;

    for p in pairs {
        match p.as_rule() {
            Rule::AttributePrefix => {
                let inner = p.into_inner().next().ok_or(ParseError::NoContentForAttributePrefix)?;
                match inner.as_rule() {
                    Rule::Default => {
                        let inner = inner.into_inner().next().ok_or(ParseError::NoDefaultCode)?;
                        default_code = Some(inner.as_str().into());
                    }
                    Rule::Required => {
                        required = true;
                    }
                    _ => Err(ParseError::InvalidAttributePrefixContent(inner.as_str().into()))?,
                }
            }
            Rule::Identifier => {
                attribute_identifier = Some(p.as_str());
            }
            Rule::AttributeValue => {
                attribute_value = Some(p.as_str());
            }
            Rule::FeatureGate => {
                feature_flag = parse_feature_gate(p.into_inner())?;
            }
            _ => Err(ParseError::InvalidAttributeContent(p.as_str().into()))?
        }
    }
    let attribute_value = attribute_value.ok_or(ParseError::NoAttributeValue)?;
    let attribute_identifier = attribute_identifier.ok_or(ParseError::NoAttributeIdentifier)?;
    Ok(Attribute {
        required,
        features: feature_flag.map(|f| vec![FeatureFlag::from(f)]).unwrap_or(vec![]),
        name: Cow::Owned(attribute_identifier.into()),
        default_code: default_code.map(|s| Cow::from(s)),
        value: Cow::Owned(attribute_value.into()),
    })
}


fn parse_feature_gate(pairs: Pairs<Rule>) -> Result<Option<&str>, ParseError> {
    let inner = pairs.into_iter().next().ok_or(ParseError::NoFeatureGate)?;
    match inner.as_rule() {
        Rule::FeatureName => {
            Ok(Some(inner.as_str()))
        }
        _ => Err(ParseError::NoFeatureName)?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file() {
        let str: &'static str = include_str!("test_data2.rs");
        let parse_result = RustSourceParser::parse_data(str).unwrap();
        assert_eq!(1, parse_result.components.len());
    }

    #[test]
    fn test_parse_test_data() {
        let str: &'static str = include_str!("test_data.rs");
        RustSourceParser::parse(Rule::File, str).unwrap();
        let parse_result = RustSourceParser::parse_data(str).unwrap();
        assert_eq!(2, parse_result.modules.len());
        assert!(parse_result.modules.iter().any(|m| &m.name == "modulea"));
        assert!(parse_result.modules.iter().any(|m| &m.name == "moduleweb" && m.feature_flag == Some(FeatureFlag("web".into()))));
        assert_eq!(2, parse_result.components.len());
        let component_web = parse_result.components.get(0).unwrap();
        assert_eq!(component_web.simple_name, Cow::Borrowed("ComponentWeb"));

        let component = parse_result.components.get(1).unwrap();
        assert_eq!(component.simple_name, Cow::Borrowed("Component"));

        assert_eq!(1, component.required_attributes.len());
        assert_eq!(component.required_attributes.get(0).unwrap().name, "required");

        assert_eq!(3, component.optional_attributes.len());

        let attribute1 = component.optional_attributes.iter().find(|p| p.name == Cow::Borrowed("my_rop")).unwrap();
        assert_eq!(attribute1.value, "MyStruct");
        assert_eq!(attribute1.default_code, None);
        assert!(attribute1.features.is_empty());

        let attribute2 = component.optional_attributes.iter().find(|p| p.name == Cow::Borrowed("props")).unwrap();
        assert_eq!(attribute2.value, "Vec<Bla>");
        assert_eq!(attribute2.default_code, Some(Cow::Borrowed("Vec::with_capacity(0)")));
        assert!(attribute2.features.is_empty());

        let attribute3 = component.optional_attributes.iter().find(|p| p.name == Cow::Borrowed("attribute_android")).unwrap();
        assert_eq!(attribute3.value, "String");
        assert_eq!(attribute3.default_code, None);
        assert_eq!(attribute3.features.len(), 1);
        let flag = attribute3.features.get(0).unwrap();
        assert_eq!(flag.0.as_str(), "android");
    }

    #[test]
    fn test_parse_test_attributes() {
        RustSourceParser::parse(Rule::Attribute, "bla: Blubb,").unwrap();
        RustSourceParser::parse(Rule::Attribute, "bla: Vec<Blubb>,").unwrap();
        RustSourceParser::parse(Rule::Attribute, "bla_blubb: &' static str,").unwrap();
        let required = "#[required]\
        bla: BLubb,";
        RustSourceParser::parse(Rule::Attribute, required).unwrap();

        let default = "#[default(Blubb::new)]\
        bla: BLubb,";
        RustSourceParser::parse(Rule::Attribute, default).unwrap();
    }
}
