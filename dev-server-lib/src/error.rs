use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Operating system {} is not known.", name))]
    UnknownOperatingSystem{name: String},

}