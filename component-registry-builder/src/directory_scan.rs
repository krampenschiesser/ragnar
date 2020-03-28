use crate::FeatureFlag;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use crate::parser::{RustSourceParser, ParseResult};
use crate::component_registry::ComponentRegistry;
use crate::parse_error::ParseError;
use crate::parse_error::ParseError::PestError;

pub fn scan_file(path: &Path) -> Result<ParseResult, ParseError> {
    use std::io::BufRead;

    let input = File::open(path)?;
    let mut buffered = BufReader::new(input);

    let mut data = String::new();
    buffered.read_to_string(&mut data)?;
    let parse_result = RustSourceParser::parse_data(&data)?;
    Ok(parse_result)
}

pub fn scan_dir(path: &Path) -> Result<ComponentRegistry, ParseError> {
    let mut queue: Vec<(PathBuf, Vec<FeatureFlag>, String)> = Vec::new();
    queue.push((path.join("lib.rs"), Vec::new(), "".into()));
    let mut all_components = Vec::new();
    loop {
        if let Some((file, flags, path)) = queue.pop() {
            let ParseResult { components, modules } = scan_file(file.as_path())?;

            components.into_iter().for_each(|mut c| {
                flags.iter().for_each(|f| c.features.push(f.clone()));
                c.qualified_name = path.clone().into();
                all_components.push(c)
            });

            for module in modules {
                let new_path = if path.is_empty(){module.name.clone()}else{format!("{}.{}", path, module.name)};
                let parent = file.parent().ok_or(ParseError::NoParentFound)?;
                let modfile = parent.join(format!("{}.rs", &module.name));
                if modfile.exists() {
                    queue.push((modfile, flags.clone(), new_path));
                } else {
                    let mod_rs_file = parent.join(&module.name).join("mod.rs");
                    if mod_rs_file.exists() {
                        queue.push((mod_rs_file, flags.clone(), new_path));
                    } else {
                        Err(ParseError::CouldNotFindModuleFile { module_name: module.name.clone(), folder: parent.to_string_lossy().into() })?;
                    }
                }
            }
        } else {
            break;
        }
    }
    Ok(ComponentRegistry{
        components: all_components,
    })
}
