use std::prelude::v1::*;

#[cfg(feature = "rest-mappings-serde")]
use serde::{Deserialize, Serialize};

use crate::util::string::AllCaps;

#[cfg_attr(
    feature = "rest-mappings-serde",
    derive(Serialize, Deserialize),
    serde(from = "String", into = "String")
)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone)]
pub enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
    Any,
    Other(AllCaps),
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        Self::from(AllCaps::from(s))
    }
}

impl From<String> for Method {
    fn from(s: String) -> Self {
        Self::from(AllCaps::from(s))
    }
}

impl From<AllCaps> for Method {
    fn from(s: AllCaps) -> Self {
        match s.as_str() {
            "ANY" => Self::Any,
            "GET" => Self::GET,
            "HEAD" => Self::HEAD,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "DELETE" => Self::DELETE,
            "CONNECT" => Self::CONNECT,
            "OPTIONS" => Self::OPTIONS,
            "TRACE" => Self::TRACE,
            "PATCH" => Self::PATCH,
            _ => Self::Other(s),
        }
    }
}

impl From<Method> for String {
    fn from(m: Method) -> Self {
        match m {
            // specialize the Other case as it is already a String
            Method::Other(s) => s.into(),
            _ => m.as_str().into(),
        }
    }
}

impl PartialEq for Method {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Any, _) | (_, Self::Any) => true,
            (Self::Other(a), Self::Other(b)) => a == b,
            (Self::GET, Self::GET)
            | (Self::HEAD, Self::HEAD)
            | (Self::POST, Self::POST)
            | (Self::PUT, Self::PUT)
            | (Self::DELETE, Self::DELETE)
            | (Self::CONNECT, Self::CONNECT)
            | (Self::OPTIONS, Self::OPTIONS)
            | (Self::TRACE, Self::TRACE)
            | (Self::PATCH, Self::PATCH) => true,
            _ => false,
        }
    }
}

impl Eq for Method {}

impl Method {
    pub fn as_str(&self) -> &str {
        match self {
            Method::Any => "ANY",
            Method::GET => "GET",
            Method::HEAD => "HEAD",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::CONNECT => "CONNECT",
            Method::OPTIONS => "OPTIONS",
            Method::TRACE => "TRACE",
            Method::PATCH => "PATCH",
            Method::Other(s) => s.as_str(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use fixtures::all_methods;

    mod fixtures {
        use super::*;

        pub fn all_methods() -> Box<[Method]> {
            Box::new([
                Method::GET,
                Method::HEAD,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::CONNECT,
                Method::OPTIONS,
                Method::TRACE,
                Method::PATCH,
                Method::Other("future_method".into()),
            ])
        }
    }

    #[test]
    fn any_matches_every_other_method() {
        let method = Method::from("any");

        for other in all_methods().iter() {
            assert_eq!(&method, other);
        }
    }

    #[test]
    fn other_only_matches_exact_other() {
        let method = Method::from("future_method");

        assert_ne!(method, Method::Other("other_future_method".into()));

        let all_methods = all_methods();
        let other = all_methods
            .iter()
            .filter(|&m| m == &method)
            .collect::<Vec<_>>();
        assert_eq!(other.len(), 1);

        assert_eq!(&method, other[0]);
    }

    #[test]
    fn equality_and_identity() {
        let all_methods = all_methods();
        for (idx, method) in all_methods.iter().enumerate() {
            // create a list of all methods except the currently processed by index (ie. not using ==)
            let other_methods: Vec<_> = all_methods
                .iter()
                .enumerate()
                .filter(|(oidx, _)| idx != *oidx)
                .map(|(_, m)| m)
                .cloned()
                .collect();

            // identity
            assert_eq!(method, &all_methods[idx]);

            for other_method in other_methods {
                assert_ne!(method, &other_method);
            }
        }
    }

    #[test]
    fn transitive_id_with_strings() {
        for method in all_methods().iter() {
            transitive_id_with_strings_check(method);
        }
    }

    #[test]
    fn transitive_id_with_method_any() {
        assert_eq!(Method::Any.as_str(), "ANY");
        assert_eq!(String::from(Method::Any), "ANY".to_string());
        assert!(matches!(Method::from("ANY"), Method::Any));
        assert!(matches!(Method::from("ANY".to_string()), Method::Any));

        transitive_id_with_strings_check(&Method::Any);
    }

    fn transitive_id_with_strings_check(method: &Method) {
        let string: String = method.clone().into();
        let other_string_method: Method = string.into();
        let other_str_method: Method = method.as_str().into();

        assert_eq!(method, &other_string_method);
        assert_eq!(method, &other_str_method);
    }
}
