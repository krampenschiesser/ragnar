use crate::ParseError;
use semver::Version;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use super::ScannedCrate;

pub fn scan_dependencies(path: &Path) -> Result<Vec<ScannedCrate>, ParseError> {
    let mut vec = Vec::new();
    for cur_path in path.read_dir().map_err(|e| ParseError::IoError { file: path.to_string_lossy().to_string(), source: e })? {
        let path = cur_path.map_err(|e| ParseError::IoError { file: path.to_string_lossy().to_string(), source: e })?;
        if path.file_name().to_string_lossy().ends_with(".d") {
            let c = get_crate(&path.path())?;
            vec.push(c);
        }
    }
    Ok(vec)
}

fn get_crate(path: &Path) -> Result<ScannedCrate, ParseError> {
    let file = File::open(path).map_err(|e| ParseError::IoError { file: path.to_string_lossy().to_string(), source: e })?;
    let mut buffered = BufReader::new(file);
    let mut line = String::new();
    buffered.read_line(&mut line).map_err(|e| ParseError::IoError { file: path.to_string_lossy().to_string(), source: e })?;
    let mut split = line.split(" ");
    split.next();
    let librs_string = split.next().ok_or_else(|| ParseError::NoLibRs { file: path.to_string_lossy().into(), line: line.clone() })?;
    let source_path = librs_string.split("lib.rs").next().ok_or_else(|| ParseError::NoLibRs { file: path.to_string_lossy().into(), line: librs_string.into() })?;
    let toml_file = Path::new(source_path).parent().unwrap().join("Cargo.toml");
    if !toml_file.exists() {
        Err(ParseError::NoTomlFile(toml_file.to_string_lossy().to_string()))?;
    }
    scan_toml_file(&toml_file, source_path.into())
}

fn scan_toml_file(path: &Path, source_path: String) -> Result<ScannedCrate, ParseError> {
    let file = File::open(path).map_err(|e| ParseError::IoError { file: path.to_string_lossy().to_string(), source: e })?;
    let mut buffered = BufReader::new(file);
    let mut content = String::new();
    buffered.read_to_string(&mut content).map_err(|e| ParseError::IoError { file: path.to_string_lossy().to_string(), source: e })?;

    let toml: TomlFile = toml::from_str(&content).map_err(|e| ParseError::TomlParseError { file: path.to_string_lossy().to_string(), source: e })?;

    let version = match Version::parse(toml.package.version.as_str()) {
        Ok(v) => Some(v),
        Err(_e) => {
            error!("Could not parse semver version from '{}' in file '{}'", toml.package.version, path.to_string_lossy());
            None
        }
    };
    Ok(ScannedCrate {
        version,
        name: toml.package.name,
        components: vec![],
        source_path: source_path.into(),
    })
}

#[derive(serde::Deserialize, Debug)]
struct TomlFile {
    package: TomlPackage
}

#[derive(serde::Deserialize, Debug)]
struct TomlPackage {
    name: String,
    version: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_debug() {
        let cwd = std::env::current_dir().unwrap();
        let dep_dir = cwd.join("target").join("debug").join("deps");
        let crates = scan_dependencies(&dep_dir).unwrap();
        assert!(crates.len() > 0);
    }
}
