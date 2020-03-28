use failure::Fail;
use pest::error::Error;

#[derive(Debug, Fail)]
pub enum ParseError {
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
    #[fail(display = "Invalid struct content {}", _0)]
    InvalidStructContent(String),
    #[fail(display = "Invalid attribute content {}", _0)]
    InvalidAttributeContent(String),
    #[fail(display = "Invalid attribute prefix content {}", _0)]
    InvalidAttributePrefixContent(String),

    #[fail(display = "Underlying pest error: {:?}", source)]
    PestError {
        #[fail(cause)]
        source: pest::error::Error<crate::parser::Rule>,
    },
    #[fail(display = "Underlying io error: {:?}", source)]
    IoError {
        #[fail(cause)]
        source: std::io::Error,
    },
}

impl From<pest::error::Error<crate::parser::Rule>> for ParseError {
    fn from(e: pest::error::Error<crate::parser::Rule>) -> Self {
        ParseError::PestError { source: e }
    }
}

impl From<std::io::Error> for ParseError {
    fn from(e: std::io::Error) -> Self {
        ParseError::IoError { source: e }
    }
}

impl ParseError {
    pub fn get_backtrace(&self) -> String {
        format!("{:?}", self.backtrace())
    }
}