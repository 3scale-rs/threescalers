use snafu::{Snafu};

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum Error {
    #[snafu(display("Error calling build function for builder"))]
    BuilderPatternBuildErr,
}
