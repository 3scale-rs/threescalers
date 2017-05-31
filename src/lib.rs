#[macro_use]
extern crate error_chain;

pub mod errors;
pub mod request;
pub mod service;
pub mod application;
pub mod user;
pub mod apicall;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
