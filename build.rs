// Detect nightly compilers and features.
use rustc_version::{
    version_meta,
    Channel,
    Version,
    VersionMeta,
};

use AvailabilityStatic::*;

// Declare features used by the project.
// Use minimum and maximum (not included) versions.
// "always"/"unknown" for minimum and "unknown" for maximum are special.
static FEATURES: &[Feature<'_>] = &[Feature { name:         "nightly",
                                                      availability: Nightly("always", "unknown"), },
                                            Feature { name:         "test",
                                                      availability: NightlyGated("always", "unknown"), },
                                            Feature { name:         "never_type",
                                                      availability: NightlyGated("1.12.0", "unknown"), },
                                            Feature { name:         "const_saturating_int_methods",
                                                      availability: Stable("1.47.0", "unknown"), }];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let version_meta = version_meta()?;

    FEATURES.iter()
            .map(|f| f.cargo_flags(&version_meta))
            .for_each(|fl_set| fl_set.iter().for_each(|fl| println!("{}", fl)));

    Ok(())
}

// *** Supporting code below ***
enum VersionSpec {
    AlwaysSupported,
    EqualOrGreater(Version),
    Between(Version, Version),
    Lower(Version),
}

impl VersionSpec {
    pub fn matches(&self, version: &Version) -> bool {
        use VersionSpec::*;

        match self {
            AlwaysSupported => true,
            EqualOrGreater(req) => version >= req,
            Lower(req) => version < req,
            Between(reqmin, reqmax) => version >= reqmin && version < reqmax,
        }
    }
}

impl From<(&str, &str)> for VersionSpec {
    fn from((min, top): (&str, &str)) -> VersionSpec {
        let min_v = Version::parse(min);
        let top_v = Version::parse(top);

        match (min, top) {
            ("unknown", "unknown") | ("always", "unknown") => VersionSpec::AlwaysSupported,
            ("unknown", _) | ("always", _) => VersionSpec::Lower(top_v.unwrap()),
            (_, "unknown") => VersionSpec::EqualOrGreater(min_v.unwrap()),
            (..) => VersionSpec::Between(min_v.unwrap(), top_v.unwrap()),
        }
    }
}

enum Availability {
    Stable(VersionSpec),
    Nightly(VersionSpec),
    NightlyGated(VersionSpec),
}

enum FeatureFlags {
    Unavailable,
    HasFeature,
    HasGatedFeature,
}

struct Feature<'a> {
    name:         &'a str,
    availability: AvailabilityStatic,
}

impl<'a> Feature<'a> {
    pub fn name(&self) -> &str {
        self.name
    }

    pub fn flags(&self, version_meta: &VersionMeta) -> FeatureFlags {
        use Availability::*;

        let version = &version_meta.semver;
        let channel = version_meta.channel;

        let availability = Availability::from(&self.availability);

        match availability {
            Stable(spec) if channel == Channel::Stable || channel == Channel::Beta => {
                if spec.matches(version) {
                    FeatureFlags::HasFeature
                } else {
                    FeatureFlags::Unavailable
                }
            }
            Nightly(spec) if channel == Channel::Nightly => {
                if spec.matches(version) {
                    FeatureFlags::HasFeature
                } else {
                    FeatureFlags::Unavailable
                }
            }
            NightlyGated(spec) if channel == Channel::Nightly => {
                if spec.matches(version) {
                    FeatureFlags::HasGatedFeature
                } else {
                    FeatureFlags::Unavailable
                }
            }
            _ => FeatureFlags::Unavailable,
        }
    }

    pub fn cargo_flags(&self, version_meta: &VersionMeta) -> Box<[String]> {
        match self.flags(version_meta) {
            FeatureFlags::HasFeature => [format!("cargo:rustc-cfg=has_{}", self.name())].into(),
            FeatureFlags::HasGatedFeature => [format!("cargo:rustc-cfg=has_{}", self.name()),
                                              format!("cargo:rustc-cfg=feature_gate_{}", self.name())].into(),
            FeatureFlags::Unavailable => [].into(),
        }
    }
}

enum AvailabilityStatic {
    Stable(&'static str, &'static str),
    NightlyGated(&'static str, &'static str),
    Nightly(&'static str, &'static str),
}

impl From<&AvailabilityStatic> for Availability {
    fn from(avs: &AvailabilityStatic) -> Availability {
        use Availability as Av;
        use AvailabilityStatic::*;

        match *avs {
            Stable(min, top) => Av::Stable((min, top).into()),
            Nightly(min, top) => Av::Nightly((min, top).into()),
            NightlyGated(min, top) => Av::NightlyGated((min, top).into()),
        }
    }
}
