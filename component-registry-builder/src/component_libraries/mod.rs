use semver::VersionReq;
use crate::ParseError;
use std::path::Path;

pub struct ComponentLibrary {
    pub name: String,
    pub version: VersionReq,
}


pub fn parse_toml_file_contents(content: &str,file_path: &Path) -> Result<Vec<ComponentLibrary>, ParseError> {
    let mut ret = Vec::new();

    let iter = content.lines()
        .filter(|l| !l.trim().starts_with("#"))
        .filter(|l| l.contains("=") && l.len() > 3)
        .filter_map(|l| {
            let mut split = l.split("=");
            let name = split.next();
            let version = split.next();
            name.and_then(|n| version.map(|v| (n.trim(), v.trim())))
        })
        .map(|(name, version)| {
            let result = VersionReq::parse(version).map_err(|source| ParseError::SemverReqError { source, string: version.into(),file: file_path.to_string_lossy().into() });
            result.map(|v| ComponentLibrary { name: name.into(), version: v })
        });
    for res in iter {
        ret.push(res?);
    }
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use semver::Version;

    #[test]
    fn test_parse_file() {
        let toml_str = r#"
            html = 5.0
            common-markup = 1.0
            "#;

        let vec = parse_toml_file_contents(toml_str, Path::new("test")).unwrap();
        assert_eq!(2, vec.len());
        let html = vec.get(0).unwrap();
        assert_eq!(html.name, "html");
        assert!(html.version.matches(&Version::new(5, 0, 0)));
        let common = vec.get(1).unwrap();
        assert_eq!(common.name, "common-markup");
        assert!(common.version.matches(&Version::new(1, 0, 0)));
    }
}
