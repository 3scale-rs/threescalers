macro_rules! description {
    () => ("threescalers")
}
macro_rules! version {
    () => (env!("CARGO_PKG_VERSION"))
}
pub const VERSION: &str = version!();
pub const USER_AGENT: &str = concat!(description!(), "/", version!());

