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

