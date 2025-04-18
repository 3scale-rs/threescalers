# threescalers

This library offers types to work with the 3scale Service Management API and
convenience implementations for some comonly used HTTP clients.

## Rust library crate for the 3scale Service Management API

![Minimum rustc version](https://img.shields.io/badge/rustc-1.65.0+-green.svg)
[![docs.rs](https://docs.rs/threescalers/badge.svg)](https://docs.rs/threescalers)
[![Build Status](https://github.com/3scale-rs/threescalers/actions/workflows/ci.yaml/badge.svg)](https://travis-ci.org/3scale-rs/threescalers)
[![codecov.io](https://codecov.io/gh/3scale-rs/threescalers/coverage.svg?branch=master)](https://codecov.io/gh/3scale-rs/threescalers/branch/master)

This library was created as a side-effect free client for 3scale's Service Management
API as exposed by [Apisonator](https://github.com/3scale/apisonator), the backend
service of the 3scale API Management platform.

A side-effect free library in this context is also referred to as `sans IO`, which
ends up describing how the requests you perform should look like and can also parse
responses you receive so you can interpret their meaning. By not tying the library
to any particular protocol client implementation you are free to use whichever works
best in each particular case.

## Minimum Supported Rust Version

No promise is made to maintain compatibility, but a best-effort policy of supporting
the MSRV announced above is kept so that we try not to bump that minimum version
unnecessarily.

Some features that pull in dependencies might require higher rustc versions based on
the dependencies' MSRV.

The current MSRV is 1.65.0, but if you enable any feature related to reqwest, the
MSRV is bumped to 1.81.0.

## no_std

`#[no_std]` support can be achieved _if_ allocations are allowed and disabling the default
feature set, but since at least one feature uses `lazy_static`, you'll want to build your
binary enabling the spinlocks feature of this dependency, named `spin_no_std`. If you don't
do this `std` will be pulled in regardless by `lazy_static` as of writing.

## Status

This library is in _beta_ state. It should be useful to create clients that will
work with 3scale, and while it may have some rough edges it is already being used
in projects that should be hitting production soon.

Please report any issues at the [issue tracker](https://github.com/3scale-rs/threescalers/issues).
