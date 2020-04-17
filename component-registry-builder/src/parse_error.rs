use failure::Fail;


#[derive(Debug, Fail)]
pub enum ParseError {
    #[fail(display = "No toml file found at path {}", _0)]
    NoTomlFile(String),
    #[fail(display = "Could not parse crate string {}, file '{}'", _0,_1)]
    WrongCrateString(String,String),
    #[fail(display = "Could not find lib rs in file {}  and line {}", file, line)]
    NoLibRs { file: String, line: String },
    #[fail(display = "Could not find module {} in folder {}. no {}.rs and no {}/mod.rs", module_name, folder, module_name, module_name)]
    CouldNotFindModuleFile { module_name: String, folder: String },
    #[fail(display = "No parent found")]
    NoParentFound,
    #[fail(display = "No file content")]
    NoFileContent,
    #[fail(display = "No attribute value")]
    NoAttributeValue,
    #[fail(display = "No attribute identifier")]
    NoAttributeIdentifier,
    #[fail(display = "No code in default block")]
    NoDefaultCode,
    #[fail(display = "No content for attribute prefix")]
    NoContentForAttributePrefix,
    #[fail(display = "No content for rename")]
    NoContentForRename,
    #[fail(display = "Could not find struct identifier")]
    CouldNotFindStructIdentifier,
    #[fail(display = "Could not parse module")]
    CouldNotParseModule,
    #[fail(display = "No feature gate")]
    NoFeatureGate,
    #[fail(display = "No feature name")]
    NoFeatureName,
    #[fail(display = "No identifier")]
    NoIdentifier,
    #[fail(display = "Invalid module content {}", _0)]
    InvalidModuleContent(String),
    #[fail(display = "Invalid component impl content {}", _0)]
    InvalidComponentImplContent(String),
    #[fail(display = "Invalid struct content {}", _0)]
    InvalidStructContent(String),
    #[fail(display = "Invalid attribute content {}", _0)]
    InvalidAttributeContent(String),
    #[fail(display = "Invalid attribute prefix content {}", _0)]
    InvalidAttributePrefixContent(String),
    #[fail(display = "Invalid rename content {}", _0)]
    InvalidRenameContent(String),

    #[fail(display = "Underlying pest error: {:?}", source)]
    PestError {
        #[fail(cause)]
        source: pest::error::Error<crate::rust_lib_parser::Rule>,
    },
    #[fail(display = "Underlying io error in file {}: {:?}", file, source)]
    IoError {
        file: String,
        #[fail(cause)]
        source: std::io::Error,
    },
    #[fail(display = "Could not parse semver version from string '{}', file='{}': {:?}", string, file, source)]
    SemverError {
        file: String,
        string: String,
        #[fail(cause)]
        source: semver::SemVerError,
    },
    #[fail(display = "Could not parse semver requirement from string '{}', file='{}': {:?}", string,file, source)]
    SemverReqError {
        file: String,
        string: String,
        #[fail(cause)]
        source: semver::ReqParseError,
    },
    #[fail(display = "Could not deserialize json: {:?}", source)]
    SerdeJsonError {
        #[fail(cause)]
        source: serde_json::Error,
    },
    #[fail(display = "Could not parse toml file='{}': {:?}", file, source)]
    TomlParseError {
        file: String,
        #[fail(cause)]
        source: toml::de::Error,
    },
}

impl From<pest::error::Error<crate::rust_lib_parser::Rule>> for ParseError {
    fn from(e: pest::error::Error<crate::rust_lib_parser::Rule>) -> Self {
        ParseError::PestError { source: e }
    }
}

impl ParseError {
    pub fn get_backtrace(&self) -> String {
        format!("{:?}", self.backtrace())
    }
}


impl From<serde_json::Error> for ParseError {
    fn from(e: serde_json::Error) -> Self {
        ParseError::SerdeJsonError {
            source: e,
        }
    }
}