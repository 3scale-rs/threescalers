use std::prelude::v1::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(from = "String", into = "String")
)]
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllCaps(String);

impl AllCaps {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<&str> for AllCaps {
    fn from(s: &str) -> Self {
        Self(s.to_ascii_uppercase())
    }
}

impl From<String> for AllCaps {
    fn from(mut s: String) -> Self {
        s.make_ascii_uppercase();
        Self(s)
    }
}

impl From<AllCaps> for String {
    fn from(a: AllCaps) -> Self {
        a.into_inner()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ensure_all_caps_always_is_fully_capitalized() {
        let mixed_case_s = "miX3d_case-str^Ng.";

        let all_caps = AllCaps::from(mixed_case_s);

        assert_eq!(all_caps.as_str(), mixed_case_s.to_ascii_uppercase());
        assert_eq!(all_caps.into_inner(), "MIX3D_CASE-STR^NG.");
    }
}
