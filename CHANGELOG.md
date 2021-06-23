# Change Log

Notable changes to threescalers will be tracked in this document.

## 0.8.0 - 2021-06-23

### Compatibility

- [__BREAKING__] This release is a breaking change from previous releases due to
  several public interfaces changing, including types that implement more but also
  less traits, and also a different amount of generic lifetime parameters.

### Added

- Support emitting the [`list_app_keys` extension](https://github.com/3scale/apisonator/blob/v3.4.3/docs/extensions.md#list_app_keys-integer)
  and parsing the results back. ([#90](https://github.com/3scale-rs/threescalers/pull/90))
- Some types for which there was an `Into` impl now also offer a `From` impl. ([#81](https://github.com/3scale-rs/threescalers/pull/81))
- New implementation with deserialization support for REST mapping rules. This is
  experimental and will likely change, possibly incompatibly, in the future. ([#78](https://github.com/3scale-rs/threescalers/pull/78))
- Specifically allow reqwest 0.10 and 0.11 to be used when enabling its feature.
  This allows for users to not be stuck either one or the other as chosen by us. ([#76](https://github.com/3scale-rs/threescalers/pull/76))

### Changed

- Minimum Supported Rust Version is `rustc 1.40.0`. ([#83](https://github.com/3scale-rs/threescalers/pull/83))
- The `ApiCall`, its `Builder` and the `Transaction` types have seen their signatures
  reduce their multiple generic lifetime parameters to just one. ([#86](https://github.com/3scale-rs/threescalers/pull/86))
- More types that could use a transparent representation now do so. ([#82](https://github.com/3scale-rs/threescalers/pull/82), ([#85](https://github.com/3scale-rs/threescalers/pull/85)))
- The `Period` enum now includes an `Other` variant that makes it non-copy and non-exhaustive. ([#97](https://github.com/3scale-rs/threescalers/pull/97))
- The `response` module has been refactored, with the `Authorization` enum variants
  renamed, refactored and with new helpers to improve ergonomics and allow reusing
  `UsageReport`s by mutating their counters. ([#89](https://github.com/3scale-rs/threescalers/pull/89), [#96](https://github.com/3scale-rs/threescalers/pull/96))

### Fixed

- The response parsing code will not panic in case malformed metrics hierarchy entry
  is passed in. ([#83](https://github.com/3scale-rs/threescalers/pull/83))

### Thanks

- [@NomadXD](https://github.com/NomadXD)
- [@rahulanand16nov](https://github.com/rahulanand16nov)
- [@unleashed](https://github.com/unleashed)

## 0.7.0 - 2020-10-28

### Compatibility

- [__BREAKING__] This release is a breaking change from previous releases due to
  public functions and types changing signatures.

### Added

- Added no_std mode. Disable default features to get it unless you enable the "std"
  feature. We now use anyhow to generate errors in fallible functions. ([#69](https://github.com/3scale-rs/threescalers/pull/69))

### Changed

- The timestamps used are now simple integers removing the complexity of using
  SystemTime implementations. ([#73](https://github.com/3scale-rs/threescalers/pull/73))
- Updated the XML parsing crate serde-xml-rs to 0.4 ([#72](https://github.com/3scale-rs/threescalers/pull/72))

### Removed

- Removed the mandatory dependencies on error_chain and the http crate. ([#71](https://github.com/3scale-rs/threescalers/pull/71))

### Thanks

- [@unleashed](https://github.com/unleashed)

## 0.6.1 - 2020-10-08

### Compatibility

- This release is again usable with newer (and hopefully older) nightlies and makes
  use of new facilities on 1.47.0. ([#70](https://github.com/3scale-rs/threescalers/pull/70))

### Thanks

- [@unleashed](https://github.com/unleashed)

## 0.6.0 - 2020-05-03

### Changed

- [__BREAKING__] Introduce typed extensions. ([#67](https://github.com/3scale-rs/threescalers/pull/67))

### Compatibility

- This release is a breaking change from the 0.5 series if you use extensions.

### Thanks

- [@unleashed](https://github.com/unleashed)

## 0.5.0 - 2020-02-11

### Changed

- [__BREAKING__] Support for http 0.1 and reqwest 0.9 series has been dropped in favor
  of the http 0.2 and reqwest 0.10 series. ([#66](https://github.com/3scale-rs/threescalers/pull/66))
- Depend internally on percent_encoding 0.2. ([#65](https://github.com/3scale-rs/threescalers/pull/65))

### Compatibility

- This release is a breaking change from the 0.4 series.

### Thanks

- [@unleashed](https://github.com/unleashed)

## 0.4.0 - 2020-02-02

### Changed

- [__BREAKING__] The Usage type now tracks string slices rather than cloning them.
  Currently this means that users will need to pass in usage data already as string
  slices rather than integer types, though helpers or a Cow type might be added
  later on. This is internally represented as a vector that the caller can access.
  ([#63](https://github.com/3scale-rs/threescalers/pull/63))

### Fixed

- Replace uses of the method [T]::into_iter() with [T]::iter(). This was deprecated
  in Rust 1.41.0. ([#59](https://github.com/3scale-rs/threescalers/pull/59))

### Compatibility

- This release is a breaking change from the 0.3 series.

### Thanks

- [@peakmorgana](https://github.com/peakmorgana)
- [@unleashed](https://github.com/unleashed)

## 0.3.0 - 2019-09-24

### Added

- The dependencies for this project are checked by LicenseFinder to be free
  software licenses compatible with this project (ie. many/most of them, it's
  a non comprehensive list so far). ([#56](https://github.com/3scale-rs/threescalers/pull/56))

### Changed

- The date parsing for XML responses now returns an error to the caller rather
  than panicking/aborting. ([#55](https://github.com/3scale-rs/threescalers/pull/55))
- [__BREAKING__] The `Timestamp` type now supports `SystemTime` types from
  before the UNIX epoch on systems that support those. ([#55](https://github.com/3scale-rs/threescalers/pull/55))

### Compatibility

- This release is a breaking change from the 0.2 series.

### Thanks

- [@PhilipGough](https://github.com/PhilipGough)
- [@unleashed](https://github.com/unleashed)

## 0.2.0 - 2019-07-19

### Added

- The curl::Easy and curl::Easy2 APIs of curl are now supported. ([#43](https://github.com/3scale-rs/threescalers/pull/43), [#48](https://github.com/3scale-rs/threescalers/pull/48), [#50](https://github.com/3scale-rs/threescalers/pull/50))
- Parsing of AuthRep responses via serde. ([#27](https://github.com/3scale-rs/threescalers/pull/27), [#39](https://github.com/3scale-rs/threescalers/pull/39), [#42](https://github.com/3scale-rs/threescalers/pull/42), [#44](https://github.com/3scale-rs/threescalers/pull/44), [#49](https://github.com/3scale-rs/threescalers/pull/49))
- Added a report example for Reqwest. ([#37](https://github.com/3scale-rs/threescalers/pull/37))

### Changed

- The trait that supported clients implement has changed. ([#47](https://github.com/3scale-rs/threescalers/pull/47))
- [__BREAKING__] The Extensions type has had breaking changes. ([#36](https://github.com/3scale-rs/threescalers/pull/36), [#49](https://github.com/3scale-rs/threescalers/pull/49))
- The ToParams trait has been made private. ([#50](https://github.com/3scale-rs/threescalers/pull/50))

### Fixed

- The correct headers will be sent with the Reqwest client. ([#36](https://github.com/3scale-rs/threescalers/pull/36))
- Stop unnecessarily cloning the body in Reqwest. ([#40](https://github.com/3scale-rs/threescalers/pull/40))

### Compatibility

- This release is a breaking change from the 0.1 series.

### Thanks

- [@davidor](https://github.com/davidor)
- [@unleashed](https://github.com/unleashed)

## 0.1.0 - 2019-06-28

### Added

- Initial release.

### Thanks

- [@davidor](https://github.com/davidor)
- [@unleashed](https://github.com/unleashed)
