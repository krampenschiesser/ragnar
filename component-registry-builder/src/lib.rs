#[macro_use]
extern crate pest_derive;

mod component_registry;
mod directory_scan;

mod parser;
mod parse_error;
mod feature_flag;

pub use component_registry::{ComponentRegistry, RegisteredComponent, Attribute, ComponentFeatureView};
pub use directory_scan::{scan_file, scan_dir};
pub use feature_flag::FeatureFlag;
pub use parse_error::ParseError;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_html_lib() {
        let cur_dir = std::env::current_dir().expect("Need current directory");
        let path = cur_dir.parent().unwrap().join("html-markup").join("src");
        println!("{:?}", path);
        let registry = scan_dir(path.as_path()).unwrap();
        assert!(!registry.components.is_empty());
    }
}