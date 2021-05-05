// This module implements HTTP mapping rules as casually specified in the 3scale documentation.
//
// There _are_ differences with how Apicast behaves, just because the description of what
// the rules do and don't isn't precise, and some existing behavior depend on the underlying
// regular expression engine. So consider this an approximation that we might want to limit
// or expand over time.
use std::prelude::v1::*;

use regex::Regex;

#[cfg(feature = "rest-mappings-serde")]
mod serde_impl;

mod method;
pub use method::Method;

mod escaping;

#[derive(Debug)]
pub enum HttpLineError {
    ParsingError,
}

#[derive(Debug, Clone)]
pub struct RestRule {
    method: Method,
    path: Regex,
    qs: Option<Vec<Regex>>,
}

impl RestRule {
    pub fn new<M: Into<Method>, S: AsRef<str>>(
        method: M,
        path_n_qs: S,
    ) -> Result<Self, escaping::Error> {
        let (path, qs) = escaping::split_path_n_qs(path_n_qs.as_ref());

        Self::with_path_n_qs(method, path, qs)
    }

    pub fn with_path_n_qs<M: Into<Method>, S: AsRef<str>>(
        method: M,
        path: S,
        qs: Option<S>,
    ) -> Result<Self, escaping::Error> {
        let path = escaping::path_regex(path.as_ref())?;
        let qs = qs
            .map(|qs| escaping::query_string_regex(qs.as_ref()))
            .transpose()?;

        Ok(Self {
            method: method.into(),
            path,
            qs,
        })
    }

    pub fn matches<S: AsRef<str>>(&self, method: &Method, path_qs: S) -> bool {
        method == &self.method && self.matches_path_with_qs(path_qs)
    }

    pub fn matches_request_line<S: AsRef<str>>(
        &self,
        http_request_line: S,
    ) -> Result<bool, HttpLineError> {
        let mut it = http_request_line.as_ref().splitn(3, ' ').take(2);
        let method = Method::from(it.next().ok_or(HttpLineError::ParsingError)?);
        let path_n_qs = it.next().ok_or(HttpLineError::ParsingError)?;

        Ok(self.matches(&method, path_n_qs))
    }

    pub fn matches_path_n_qs<S: AsRef<str>>(&self, path: S, qs: Option<S>) -> bool {
        self.qs
            .as_deref()
            .map(|qs_regexes| {
                let mut kvs = qs
                    .as_ref()
                    .map(AsRef::as_ref)
                    .unwrap_or("")
                    .split('&')
                    .collect::<Vec<_>>();

                qs_regexes.iter().all(|regex| {
                    kvs.iter()
                        .enumerate()
                        .find_map(
                            |(idx, &kv)| {
                                if regex.is_match(kv) {
                                    Some(idx)
                                } else {
                                    None
                                }
                            },
                        )
                        .map(|idx| {
                            kvs.remove(idx);
                            true
                        })
                        .unwrap_or(false)
                })
            })
            .unwrap_or(true)
            && self
                .path
                .is_match(escaping::coalesce_chars(path.as_ref(), '/').as_str())
    }

    pub fn matches_path_with_qs<S: AsRef<str>>(&self, path_qs: S) -> bool {
        let (path, qs) = escaping::split_path_n_qs(path_qs.as_ref());

        self.matches_path_n_qs(path, qs)
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    // String form of the path + query string pattern
    pub fn pattern(&self) -> String {
        let mut pattern = self.path.as_str()[2..].replace(escaping::PATH_VALUE_REGEX_S, "{_}");

        if let Some(qs) = self.qs.as_deref() {
            let mut qs = qs
                .iter()
                .map(|kv| kv.as_str().replace(escaping::QS_VALUE_REGEX_S, "{_}"));

            if let Some(first) = qs.next() {
                pattern.push('?');
                pattern.push_str(&first.as_str()[2..]);

                pattern = qs.fold(pattern, |mut acc, re| {
                    acc.push('&');
                    acc.push_str(&re.as_str()[2..]);
                    acc
                });
            }
        }

        pattern
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use helpers::random_method;

    mod helpers {
        use super::Method;
        use rand::prelude::SliceRandom as _;

        pub fn random_method() -> Method {
            [
                Method::Any,
                Method::GET,
                Method::HEAD,
                Method::POST,
                Method::CONNECT,
                Method::DELETE,
                Method::OPTIONS,
                Method::PUT,
                Method::TRACE,
                Method::PATCH,
                Method::Other("future_method".into()),
            ]
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone()
        }
    }

    #[test]
    fn match_simple_case() -> Result<(), escaping::Error> {
        let method = random_method();
        let mr = RestRule::new(method.clone(), "/?required=1")?;

        assert!(mr
            .matches_request_line(
                format!(
                    "{} {} HTTP/1.1",
                    method.as_str(),
                    "/test?optional=1&required=1&other=1"
                )
                .as_str()
            )
            .unwrap(),);

        Ok(())
    }

    #[test]
    fn match_simple_cases() -> Result<(), escaping::Error> {
        let mr = RestRule::new(Method::Any, "/")?;

        let path_w_qs = [
            ("/", true),
            ("/?", true),
            ("/?a", true),
            ("/?a=", true),
            ("/?a=1", true),
            ("/?a=1&", true),
            ("/?a=1&b=2", true),
            ("/some/path", true),
            ("/some/path/", true),
            ("/some/path/?a=1&b=2", true),
        ];

        for (pnqs, expected) in path_w_qs.iter() {
            let (path, qs) = escaping::split_path_n_qs(pnqs);
            let method = helpers::random_method();

            assert_eq!(mr.matches(&method, pnqs), *expected);
            assert_eq!(mr.matches_path_with_qs(pnqs), *expected);
            assert_eq!(mr.matches_path_n_qs(path, qs), *expected);
            assert_eq!(
                mr.matches_request_line(format!("{} {} HTTP/1.1", method.as_str(), pnqs).as_str())
                    .unwrap(),
                *expected
            );
        }

        Ok(())
    }

    #[test]
    fn match_edge_cases() -> Result<(), escaping::Error> {
        let path_n_qs = "/auto?maybe_empty=&w=hello&color={color}";
        let mr = RestRule::new("any", path_n_qs)?;

        let path_w_qs = [
            ("/", false),
            ("/auto", false),
            ("/auto?", false),
            ("/auto?w=hello&color=red&maybe_empty", false),
            ("/auto?w=hello&color=red&maybe_empty=", true),
            ("/auto-matic?w=hello&maybe_empty=&color=green", true),
            ("/auto?maybe_empty=its-full-now&color=blue&w=hello", true),
            ("/auto-matic?color=black&w=hello&maybe_empty=&", true),
            ("/auto-matic?w=hello&color&maybe_empty=", false),
        ];

        for (pnqs, expected) in path_w_qs.iter() {
            let (path, qs) = escaping::split_path_n_qs(pnqs);
            let method = helpers::random_method();

            assert_eq!(mr.matches(&method, pnqs), *expected);
            assert_eq!(mr.matches_path_with_qs(pnqs), *expected);
            assert_eq!(mr.matches_path_n_qs(path, qs), *expected);
            assert_eq!(
                mr.matches_request_line(format!("{} {} HTTP/1.1", method.as_str(), pnqs).as_str())
                    .unwrap(),
                *expected
            );
        }

        Ok(())
    }

    #[test]
    fn match_combined_cases() -> Result<(), escaping::Error> {
        use itertools::Itertools as _;

        let args = [r"fmt={fmt}", r"l{an}g={code}", r"s=1", r"t=$9"];

        for permutation in args.iter().permutations(args.len()) {
            let qs = permutation.iter().join("&");
            let path_n_qs = format!("{}?{}", r"/abc/v{version}/id\$$", qs);
            let mr = RestRule::new("aNY", path_n_qs.as_str())?;

            let path_w_qs = [
                ("/abc/v1/id$?fmt=html&lang=ca&s=1&t=$9", true),
                ("///abc/v1//id$?fmt=html&lang=ca&s=1&t=$9", true),
                ("/abc/v1/v2/id$?fmt=html&lang=ca&s=1&t=$9", false),
                ("/abc/v1./id$?fmt=html&lang=ca&other=70&s=1&z=2&t=$9", true),
                ("/abc//v2/id$?misc=1&t=$9&fmt=html&z=2&s=1&leng=en", true),
                ("/abc/v2/id$?misc=1&t=$998&fmt=html&z=2&s=1&l.g=ca", true),
                ("/abc/v1.1/id$?missing_required_params=1", false),
                ("/abc/v1/id$?fmt=html&lang=ca&other=70&s=2&z=1&t=$9", false),
                ("/abc/v1/id$?fmt=json&lang=ca&other=70&z=2", false),
                ("/abc/v1/id?fmt=json&lang=ca&other=70&s=1&z=2&t=$9", false),
            ];

            for (pnqs, expected) in path_w_qs.iter() {
                let (path, qs) = escaping::split_path_n_qs(pnqs);
                let method = helpers::random_method();

                assert_eq!(mr.matches(&method, pnqs), *expected);
                assert_eq!(mr.matches_path_with_qs(pnqs), *expected);
                assert_eq!(mr.matches_path_n_qs(path, qs), *expected);
                assert_eq!(
                    mr.matches_request_line(
                        format!("{} {} HTTP/1.1", method.as_str(), pnqs).as_str()
                    )
                    .unwrap(),
                    *expected
                );
            }
        }

        Ok(())
    }
}
