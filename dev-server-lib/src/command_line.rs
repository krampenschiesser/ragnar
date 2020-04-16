use structopt::StructOpt;
use std::str::FromStr;
use crate::error::Error;

#[derive(Debug, StructOpt)]
#[structopt(name = "ragnar-dev-server", about = "The development server for ragnar")]
pub struct CommandLineOptions {
    /// File name: only required when `out` is set to `file`
    #[structopt(short = "s", long = "os")]
    operating_system: OperatingSystem,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OperatingSystem {
    Windows,
    Linux,
    Mac,
    Web,
    Ios,
    Android,
}
impl Default for OperatingSystem {
    fn default() -> Self {
        OperatingSystem::Web
    }
}
impl FromStr for OperatingSystem {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Windows" | "windows" => Ok(OperatingSystem::Windows),
            "Linux" | "linux" => Ok(OperatingSystem::Linux),
            "Mac" | "mac" => Ok(OperatingSystem::Mac),
            "IOs" | "ios" | "Ios"=> Ok(OperatingSystem::Ios),
            "Android" | "android" => Ok(OperatingSystem::Android),
            "Web" | "web" => Ok(OperatingSystem::Web),
            _ => Err(Error::UnknownOperatingSystem {name: s.into()})
        }
    }
}