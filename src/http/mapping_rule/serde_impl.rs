use std::prelude::v1::*;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use super::{escaping, RestRule};

fn convert_escaping_error<E: de::Error>(ee: escaping::Error) -> E {
    match ee {
        escaping::Error::RegexError(e) => de::Error::custom(format!("regex error: {}", e)),
        escaping::Error::RegexTooBig => de::Error::custom("regex requires too much memory"),
    }
}

impl<'de> Deserialize<'de> for RestRule {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Method,
            Pattern,
        }

        struct MappingRuleVisitor;

        impl<'de> de::Visitor<'de> for MappingRuleVisitor {
            type Value = RestRule;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter
                    .write_str("a mapping rule with a method and a path and query string pattern")
            }

            fn visit_seq<V: de::SeqAccess<'de>>(self, mut seq: V) -> Result<RestRule, V::Error> {
                let method: &str = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let pattern: &str = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let mapping_rule =
                    RestRule::new(method, pattern).map_err(convert_escaping_error)?;

                Ok(mapping_rule)
            }

            fn visit_map<V: de::MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let mut method = None;
                let mut pattern = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Method => {
                            if method.is_some() {
                                return Err(de::Error::duplicate_field("method"));
                            }
                            method = Some(map.next_value()?);
                        }
                        Field::Pattern => {
                            if pattern.is_some() {
                                return Err(de::Error::duplicate_field("pattern"));
                            }
                            pattern = Some(map.next_value()?);
                        }
                    }
                }
                let method: &str = method.ok_or_else(|| de::Error::missing_field("method"))?;
                let pattern: &str = pattern.ok_or_else(|| de::Error::missing_field("pattern"))?;

                let mapping_rule =
                    RestRule::new(method, pattern).map_err(convert_escaping_error)?;

                Ok(mapping_rule)
            }
        }

        deserializer.deserialize_struct("MappingRule", &["method", "pattern"], MappingRuleVisitor)
    }
}

impl Serialize for RestRule {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("MappingRule", 2)?;
        state.serialize_field("method", self.method().as_str())?;
        state.serialize_field("pattern", self.pattern().as_str())?;
        state.end()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use fixtures::JSON;

    mod fixtures {
        pub(super) const JSON: &str = r#"{
            "method": "get",
            "pattern": "//some/{product}/id?n={id}&order=asc"
        }"#;
    }

    #[test]
    fn deserialize() -> Result<(), serde_json::Error> {
        let mapping_rule: RestRule = serde_json::from_str(JSON)?;

        assert_eq!(mapping_rule.method().as_str(), "GET");

        let path_pattern = mapping_rule.path.as_str();
        let path_expected = format!(
            r"{}/some/{}/id",
            super::escaping::START_RE,
            super::escaping::PATH_VALUE_REGEX_S
        );

        assert_eq!(path_pattern, path_expected.as_str());

        let qs_pattern = mapping_rule.qs.as_ref();
        assert!(qs_pattern.is_some());

        let qs_pattern = qs_pattern
            .unwrap()
            .iter()
            .map(regex::Regex::to_string)
            .collect::<Vec<_>>()
            .join("&");

        let qs_expected = format!(
            r"{start_re}n={qs_re}&{start_re}order=asc",
            start_re = super::escaping::START_RE,
            qs_re = super::escaping::QS_VALUE_REGEX_S
        );

        assert_eq!(qs_pattern.as_str(), qs_expected.as_str());

        Ok(())
    }

    #[test]
    fn serialize() -> Result<(), serde_json::Error> {
        use regex::Regex;

        let mapping_rule = RestRule::new("get", "///some/{product}//id$?n={id}&order=asc").unwrap();
        let json = serde_json::to_string(&mapping_rule)?;

        let other: RestRule = serde_json::from_str(json.as_str())?;
        assert_eq!(mapping_rule.method(), other.method());
        assert_eq!(mapping_rule.pattern(), other.pattern());
        assert_eq!(
            mapping_rule
                .qs
                .as_ref()
                .unwrap()
                .iter()
                .map(Regex::as_str)
                .collect::<Vec<_>>(),
            other
                .qs
                .as_ref()
                .unwrap()
                .iter()
                .map(Regex::as_str)
                .collect::<Vec<_>>()
        );

        let expected_json = r#"{"method":"GET","pattern":"/some/{_}/id$?n={_}&order=asc"}"#;
        assert_eq!(json, expected_json);

        Ok(())
    }
}
