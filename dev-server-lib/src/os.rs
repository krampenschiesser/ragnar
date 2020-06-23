use crate::command_line::OperatingSystem;

#[cfg(feature = "web")]
pub static OPERATING_SYSTEM: OperatingSystem = OperatingSystem::Web;
#[cfg(feature = "ios")]
pub static OPERATING_SYSTEM: OperatingSystem = OperatingSystem::Ios;
#[cfg(feature = "android")]
pub static OPERATING_SYSTEM: OperatingSystem = OperatingSystem::Android;
#[cfg(feature = "windows")]
pub static OPERATING_SYSTEM: OperatingSystem = OperatingSystem::Windows;
#[cfg(feature = "linux")]
pub static OPERATING_SYSTEM: OperatingSystem = OperatingSystem::Linux;
#[cfg(feature = "mac")]
pub static OPERATING_SYSTEM: OperatingSystem = OperatingSystem::Mac;
