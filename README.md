# threescalers

## Rust library crate for the 3scale Service Management API

![Minimum rustc version](https://img.shields.io/badge/rustc-1.40.0+-green.svg)
[![docs.rs](https://docs.rs/threescalers/badge.svg)](https://docs.rs/threescalers)
[![Build Status](https://travis-ci.org/3scale-rs/threescalers.svg?branch=master)](https://travis-ci.org/3scale-rs/threescalers)
[![Build status](https://ci.appveyor.com/api/projects/status/gf7okt6bbiqf2l1d/branch/master?svg=true)](https://ci.appveyor.com/project/3scale-rs/threescalers/branch/master)
[![codecov.io](https://codecov.io/gh/3scale-rs/threescalers/coverage.svg?branch=master)](https://codecov.io/gh/3scale-rs/threescalers/branch/master)

[Nightly (master) documentation](https://3scale-rs.github.io/threescalers/)

This library offers types to work with the 3scale Service Management API and
convenience implementations for some comonly used HTTP clients.

## Minimum Supported Rust Version

No promise is made to maintain compatibility, but a best-effort policy of supporting the MSRV announced above is kept so that we try not to bump that minimum version unnecessarily.
Some features that pull in dependencies might require higher rustc versions based on the dependencies' MSRV.

## Status

This library is in an early _alpha_ state. Please don't use if you fear that it
could eat your laundry. And it will most certainly eat it!
