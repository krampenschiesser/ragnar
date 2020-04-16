use pest::Parser;
use crate::FeatureFlag;
use pest::iterators::Pairs;
use crate::parse_error::ParseError;
use crate::component_registry::{RegisteredComponent, Attribute, RenderType};

#[derive(Parser)]
#[grammar = "rust_lib_parser/grammar.pest"]
pub struct RustSourceParser;

pub struct DeclaredModule {
    pub name: String,
    pub feature_flag: Option<FeatureFlag>,
}

pub struct ParseResult {
    pub modules: Vec<DeclaredModule>,
    pub components: Vec<RegisteredComponent>,
}

pub struct ComponentImplementation {
    pub render_type: RenderType,
    pub name: String,
}

impl RustSourceParser {
    pub fn parse_data(data: &str) -> Result<ParseResult, ParseError> {
        let mut parse_result = ParseResult {
            modules: Vec::new(),
            components: Vec::new(),
        };
        let mut impls: Vec<ComponentImplementation> = Vec::new();

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
                Rule::ComponentImpl => {
                    let my_impl = parse_impl(p.into_inner())?;
                    impls.push(my_impl);
                }
                _ => {}
            }
        };
        for implementation in impls {
            if let Some(component) = parse_result.components.iter_mut().find(|c|c.simple_name == implementation.name) {
                component.render_type = implementation.render_type
            }
        }
        Ok(parse_result)
    }
}

fn parse_impl(pairs: Pairs<Rule>) -> Result<ComponentImplementation, ParseError> {
    let mut render_type = RenderType::None;
    let mut identifier = None;

    for p in pairs {
        match p.as_rule() {
            Rule::App => {
                render_type = RenderType::App;
            }
            Rule::Local => {
                render_type = RenderType::Local;
            }
            Rule::Native => {
                render_type = RenderType::Native;
            }
            Rule::Identifier => {
                identifier = Some(p.as_str());
            }
            _ => Err(ParseError::InvalidComponentImplContent(p.as_str().into()))?
        }
    }
    if let Some (identifier) = identifier {
        Ok(ComponentImplementation {
            name: identifier.into(),
            render_type,
        })
    }else {
        Err(ParseError::NoIdentifier)
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
            simple_name: identifier.into(),
            required_attributes,
            optional_attributes,
            features,
            render_type: RenderType::None,
        })
    } else {
        Err(ParseError::CouldNotFindStructIdentifier)
    }
}

fn parse_attribute(pairs: Pairs<Rule>) -> Result<Attribute, ParseError> {
    let mut required = false;
    let mut delegated = false;
    let mut default_code: Option<String> = None;
    let mut feature_flag = None;
    let mut attribute_value = None;
    let mut attribute_identifier = None;
    let mut rename: Option<String> = None;

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
            Rule::Rename => {
                let inner = p.into_inner().next().ok_or(ParseError::NoContentForRename)?;
                match inner.as_rule() {
                    Rule::AnyString => {
                        // panic!("inner {:?}", inner);
                        // let inner = inner.into_inner().next().ok_or(ParseError::NoContentForRename)?;
                        rename = Some(inner.as_str().into());
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
            Rule::Delegated => {
                delegated = true;
            }
            _ => Err(ParseError::InvalidAttributeContent(p.as_str().into()))?
        }
    }
    let attribute_value = attribute_value.ok_or(ParseError::NoAttributeValue)?;
    let mut view_name: String = attribute_identifier.ok_or(ParseError::NoAttributeIdentifier)?.into();
    let code_name: String = view_name.clone();
    if let Some(rename) = rename {
        view_name = rename;
    }
    let sub_component = if delegated { Some(attribute_value.clone().into()) } else { None };
    Ok(Attribute {
        sub_component,
        required,
        features: feature_flag.map(|f| vec![FeatureFlag::from(f)]).unwrap_or(vec![]),
        name_view: view_name,
        name_code: code_name,
        default_code,
        value: attribute_value.into(),
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
    use std::borrow::Cow;

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
        assert_eq!(3, parse_result.components.len());
        let component_web = parse_result.components.get(0).unwrap();
        assert_eq!(component_web.simple_name, Cow::Borrowed("ComponentWeb"));

        let component = parse_result.components.get(1).unwrap();
        assert_eq!(component.simple_name, Cow::Borrowed("Component"));

        assert_eq!(1, component.required_attributes.len());
        assert_eq!(component.required_attributes.get(0).unwrap().name, "required");

        assert_eq!(5, component.optional_attributes.len());

        let attribute1 = component.optional_attributes.iter().find(|p| p.name == Cow::Borrowed("my_rop")).unwrap();
        assert_eq!(attribute1.value, "MyStruct");
        assert_eq!(attribute1.default_code, None);
        assert!(attribute1.features.is_empty());

        let attribute2 = component.optional_attributes.iter().find(|p| p.name == Cow::Borrowed("props")).unwrap();
        assert_eq!(attribute2.value, "Vec<Bla>");
        assert_eq!(attribute2.default_code, Some("Vec::with_capacity(0)".into()));
        assert!(attribute2.features.is_empty());

        let attribute3 = component.optional_attributes.iter().find(|p| p.name == Cow::Borrowed("attribute_android")).unwrap();
        assert_eq!(attribute3.value, "String");
        assert_eq!(attribute3.default_code, None);
        assert_eq!(attribute3.features.len(), 1);
        let flag = attribute3.features.get(0).unwrap();
        assert_eq!(flag.0.as_str(), "android");

        //renamed
        let attribute4 = component.optional_attributes.iter().find(|p| p.name == Cow::Borrowed("blubb")).unwrap();

        let attribute5 = component.optional_attributes.iter().find(|p| p.name == Cow::Borrowed("sub")).unwrap();
        assert_eq!(attribute5.sub_component, Some(String::from("Sub")))
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
