use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Operating system {name} is not known.")]
    UnknownOperatingSystem{name: String},

}