#![feature(test)]
extern crate test;

pub mod errors;
pub mod request;
pub mod service;
pub mod application;
pub mod user;
pub mod usage;
pub mod apicall;
pub mod http;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
