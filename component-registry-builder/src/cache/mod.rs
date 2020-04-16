use std::fs::File;
use std::io::{BufReader, Read};

use crate::{ParseError, RegisteredComponent, scan_dir, FeatureFlag, ComponentFeatureView};
use crate::cache::scan::scan_dependencies;
use crate::cache::scanned_crate::ScannedCrate;
use crate::component_libraries::{parse_toml_file_contents, ComponentLibrary};
use std::path::Path;
use std::collections::LinkedList;

mod scan;
mod scanned_crate;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ComponentCache {
    main_lib: Vec<RegisteredComponent>,
    crates: Vec<ScannedCrate>,
}

impl ComponentCache {
    pub fn new() -> Result<ComponentCache, ParseError> {
        let cwd = std::env::current_dir().map_err(|e| ParseError::IoError { file: "NO_FILE".into(), source: e })?;
        let cache_file = cwd.join("cache.json");
        let cache_file_fullpath = cache_file.to_string_lossy().to_string();
        let mut cache: ComponentCache = if cache_file.exists() {
            let file = File::open(cache_file).map_err(|e| ParseError::IoError { file: cache_file_fullpath.clone(), source: e })?;
            let mut buf_reader = BufReader::new(file);
            let mut content = String::new();
            buf_reader.read_to_string(&mut content).map_err(|e| ParseError::IoError { file: cache_file_fullpath.clone(), source: e })?;
            serde_json::from_str(&content)?
        } else {
            let main_lib = scan_dir(cwd.join("src").as_path(), "")?;
            ComponentCache {
                main_lib,
                crates: Vec::new(),
            }
        };

        let dep_dir = cwd.join("target").join("debug").join("deps");
        let dep_dir = if !dep_dir.exists() {
            cwd.join("target").join("release").join("deps")
        } else {
            dep_dir
        };
        let crate_defs = scan_dependencies(dep_dir.as_path())?;

        let mut toml_files = LinkedList::new();
        toml_files.push_back(cwd.join("component.deps"));
        while !toml_files.is_empty() {
            let toml_file = toml_files.pop_front().unwrap();
            let component_libs = parse_toml_file(toml_file.as_path())?;
            for c in &crate_defs {
                let register_crate = if let Some(version) = &c.version {
                    component_libs.iter().any(|cl| cl.name == c.name && cl.version.matches(version))
                } else {
                    true
                };
                let already_parsed = register_crate && cache.crates.iter().find(|o| o.name == c.name && o.version == c.version).is_some();
                if register_crate && !already_parsed {
                    toml_files.push_back(Path::new(&c.source_path).parent().unwrap().join("component.deps"));
                    let src_of_crate = Path::new(&c.source_path);
                    let components = scan_dir(src_of_crate, &c.name)?;
                    let mut my_crate: ScannedCrate = c.clone();
                    my_crate.components = components;
                    cache.crates.push(my_crate);
                }
            }
        }
        Ok(cache)
    }

    pub fn get_component<'a, 'b>(&'a self, expected_features: &'b Vec<FeatureFlag>, name: &'b str) -> Option<ComponentFeatureView<'a>> {
        let found = self.main_lib.iter().find(|c| c.simple_name.to_ascii_lowercase() == name.to_ascii_lowercase() || c.qualified_name == name).and_then(|c| c.get_feature_view(expected_features));
        if found.is_some() {
            found
        } else {
            for c in &self.crates {
                let found = c.components.iter().find(|c| c.simple_name.to_ascii_lowercase() == name.to_ascii_lowercase() || c.qualified_name == name).and_then(|c| c.get_feature_view(expected_features));
                if found.is_some() {
                    return found;
                }
            }
            None
        }
    }
}

fn parse_toml_file(toml_file: &Path) -> Result<Vec<ComponentLibrary>, ParseError> {
    if toml_file.exists() {
        let file = File::open(toml_file).map_err(|e| ParseError::IoError { file: toml_file.to_string_lossy().to_string(), source: e })?;
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();
        buf_reader.read_to_string(&mut content).map_err(|e| ParseError::IoError { file: toml_file.to_string_lossy().to_string(), source: e })?;

        let component_libs = parse_toml_file_contents(&content, toml_file)?;
        Ok(component_libs)
    } else {
        Ok(vec![])
    }
}