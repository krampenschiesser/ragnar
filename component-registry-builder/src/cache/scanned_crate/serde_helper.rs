use semver::Version;

pub(crate) fn serialize_semver<S>(version: &Option<Version>, s: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
    if let Some(version) = version {
        s.serialize_some(format!("{}", version).as_str())
    } else {
        s.serialize_none()
    }
}

pub(crate) fn deserialize_semver<'de, D>(deserializer: D) -> Result<Option<Version>, D::Error>
    where D: serde::de::Deserializer<'de> {
    use serde::de::Deserialize;
    let val: Option<String> = Option::deserialize(deserializer)?;
    // let string = String::deserialize(deserializer)?;
    if let Some(string) = val {
        match Version::parse(&string) {
            Ok(v) => {
                Ok(Some(v))
            }
            Err(e) => {
                Err(serde::de::Error::custom(format!("{}", e)))
            }
        }
    } else {
        Ok(None)
    }
}