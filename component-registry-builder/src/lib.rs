#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate log;

mod component_registry;
mod directory_scan;

mod parse_error;
mod feature_flag;
mod rust_lib_parser;
mod cache;
mod component_libraries;

pub use component_registry::{RegisteredComponent, Attribute, ComponentFeatureView,RenderType};
pub use directory_scan::{scan_file, scan_dir};
pub use feature_flag::FeatureFlag;
pub use parse_error::ParseError;
pub use cache::ComponentCache;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_html_lib() {
        let cur_dir = std::env::current_dir().expect("Need current directory");
        let path = cur_dir.parent().unwrap().join("html-markup").join("src");
        println!("{:?}", path);
        let registry = scan_dir(path.as_path()).unwrap();
        assert!(!registry.is_empty());
    }
}