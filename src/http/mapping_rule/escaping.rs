use std::prelude::v1::*;

use regex::Regex;

#[derive(Debug)]
pub enum Error {
    // Error working with or trying to build a regex
    RegexError(regex::Error),
    // Memory requirements too big
    RegexTooBig,
}

impl From<regex::Error> for Error {
    fn from(e: regex::Error) -> Self {
        Self::RegexError(e)
    }
}

// Regex to use when finding a placeholder in a path rule.
pub(super) const PATH_VALUE_REGEX_S: &str = r"[0-9a-zA-Z_\-.~%!$&'()*+,;=@:]+";
// Same for query string, but this one just avoids the ampersand character.
pub(super) const QS_VALUE_REGEX_S: &str = r"[0-9a-zA-Z_\-.~%!$'()*+,;=@:]+";
// Implicit anchor to start of string used in these expressions.
pub(super) const START_RE: &str = r"\A";

// The regular expression that defines how a placeholder looks.
lazy_static::lazy_static! {
    // panic: this literal string is a valid regular expression, so won't panic.
    static ref PLACEHOLDER_REGEX: Regex = Regex::new(r"\{.+?\}").unwrap();
}

pub(super) fn query_string_regex(s: &str) -> Result<Vec<Regex>, Error> {
    let kv_regex_s = PLACEHOLDER_REGEX
        .split(s)
        .map(|literal| {
            literal
                .split('&')
                .map(regex::escape)
                .reduce(|mut acc, escaped_part| {
                    acc.push('&');
                    acc.push_str(escaped_part.as_str());
                    acc
                })
                // panic: can't panic because String::split always return 1 or more elements in the iterator, so reduce always returns Some()
                .unwrap()
        })
        .reduce(|mut acc, escaped| {
            acc.push_str(QS_VALUE_REGEX_S);
            acc.push_str(escaped.as_str());
            acc
        })
        // panic: can't panic because Regex::split always return 1 or more elements in the iterator, so reduce always returns Some()
        .unwrap();

    let mut kv_iter = kv_regex_s.split('&');

    kv_iter.try_fold(
        Vec::with_capacity(core::cmp::max(kv_iter.size_hint().0, 8)),
        |mut acc, kv_literal| {
            let mut kv = START_RE.to_string();
            kv.push_str(kv_literal);
            Regex::new(kv.as_str())
                .map(|regex| {
                    acc.push(regex);
                    acc
                })
                .map_err(Error::from)
        },
    )
}

// Apicast path matching rule
//
// 1. Remove duplicate '/'s from the given pattern because nginx (maybe
//    others?) removes them.
// 2. Take _anything_ in between characters { } _lazily_ and replace with a
//    regex that takes a superset of alphanumeric characters.
// 3. Ensure that a terminating $ character means an exact match. (ie. don't
//    escape the remaining literal text)
pub(super) fn path_regex(path: &str) -> Result<Regex, Error> {
    let path_without_dup_fslashes = coalesce_chars(path, '/');
    let regex_literal = PLACEHOLDER_REGEX
        .split(path_without_dup_fslashes.as_str())
        .map(|literal| literal.to_string()) // No regex escaping!
        .reduce(|mut acc, literal| {
            acc.push_str(PATH_VALUE_REGEX_S);
            acc.push_str(literal.as_str());
            acc
        })
        // panic: can't panic because Regex::split always return 1 or more elements in the iterator, so reduce always returns Some()
        .unwrap();

    let required_capacity = regex_literal
        .len()
        .checked_add(START_RE.len())
        .ok_or(Error::RegexTooBig)?;

    let mut final_regex = String::with_capacity(required_capacity);

    final_regex.push_str(START_RE);
    final_regex.push_str(&regex_literal);

    Ok(Regex::new(final_regex.as_str())?)
}

pub(super) fn split_path_n_qs(s: &str) -> (&str, Option<&str>) {
    s.find('?')
        .map(|idx| (&s[..idx], Some(&s[idx + 1..])))
        .unwrap_or((s, None))
}

pub(super) fn coalesce_chars(s: &str, coalescing_char: char) -> String {
    enum Last {
        Missed,
        Matched,
    }

    let mut last = Last::Missed;

    s.chars()
        .fold(String::with_capacity(s.len()), |mut acc, c| {
            if c == coalescing_char {
                if let Last::Missed = last {
                    acc.push(c);
                    last = Last::Matched;
                }
            } else {
                acc.push(c);
                last = Last::Missed;
            }

            acc
        })
}

#[cfg(test)]
mod test {
    use super::*;

    mod coalesce_chars {
        use super::*;

        #[test]
        fn remove_duped_forward_slashes() -> Result<(), Error> {
            let pattern = "/a//b///c////d/////e/";
            assert_eq!(coalesce_chars(pattern, '/').as_str(), "/a/b/c/d/e/");

            Ok(())
        }
    }

    mod split_path_n_qs {
        use super::*;

        #[test]
        fn strips_question_mark() -> Result<(), Error> {
            let pattern = "/abc?param1=1&param2=2";
            let (path, qs) = split_path_n_qs(pattern);

            assert_eq!(path, "/abc");
            assert_eq!(qs, Some("param1=1&param2=2"));

            Ok(())
        }
    }

    mod query_string_regex {
        use super::*;

        #[test]
        fn builds_correct_qs_param_regexes() -> Result<(), Error> {
            let qs_patterns = "fmt={fmt}&hardcoded=1&lang{num}={lang}";
            let regexes = query_string_regex(qs_patterns)?;
            let regexes_s = regexes.iter().map(Regex::as_str).collect::<Vec<_>>();

            assert!(regexes_s.contains(&format!(r"{}fmt={}", START_RE, QS_VALUE_REGEX_S).as_str()));
            assert!(regexes_s.contains(&format!(r"{}hardcoded=1", START_RE).as_str()));
            assert!(regexes_s.contains(
                &format!(r"{s}lang{qs}={qs}", s = START_RE, qs = QS_VALUE_REGEX_S).as_str()
            ));

            Ok(())
        }

        #[test]
        fn matches_qs_parameters() -> Result<(), Error> {
            let qs_patterns = ["fmt={fmt}", "hardcoded=1", "lang{num}={lang}"];
            let qs_examples = ["fmt=json", "hardcoded=1", "lang_1=ca"];
            let qs_counter_ex = ["fmt-json", "hardcoded=", "a_lang_1=ca"];

            let regexes = query_string_regex(qs_patterns.join("&").as_str())?;

            for regex in regexes {
                assert!(qs_examples.iter().any(|&qs| { regex.is_match(qs) }));
                assert!(qs_counter_ex.iter().all(|&qs| { !regex.is_match(qs) }));
            }

            Ok(())
        }
    }

    mod path_regex {
        use super::*;

        #[test]
        fn match_fail() -> Result<(), Error> {
            let pattern = "/abc";
            let regex = path_regex(pattern)?;

            assert!(!regex.is_match("/aaa"));

            Ok(())
        }

        #[test]
        fn match_prefix() -> Result<(), Error> {
            let pattern = "/abc";
            let regex = path_regex(pattern)?;

            assert!(regex.is_match("/abc"));
            assert!(regex.is_match("/abcd"));

            Ok(())
        }

        #[test]
        fn match_special_chars() -> Result<(), Error> {
            let pattern = "/foo/{wildcard}/bar";
            let regex = path_regex(pattern)?;

            assert!(regex.is_match("/foo/a@b/bar"));
            assert!(regex.is_match("/foo/a:b/bar"));
            assert!(regex.is_match("/foo/a%b/bar"));
            assert!(regex.is_match("/foo/a$b/bar"));
            assert!(regex.is_match("/foo/a()b/bar"));

            Ok(())
        }

        #[test]
        fn match_exact() -> Result<(), Error> {
            let pattern = "/abc$";
            let regex = path_regex(pattern)?;

            assert!(regex.is_match("/abc"));
            assert!(!regex.is_match("/abcd"));

            Ok(())
        }

        #[test]
        fn match_dollar_sign_at_end() -> Result<(), Error> {
            let pattern = r"/abc\$";
            let regex = path_regex(pattern)?;

            assert!(regex.is_match("/abc$"));
            assert!(!regex.is_match("/abcd"));

            Ok(())
        }

        #[test]
        fn match_double_forward_slashes() -> Result<(), Error> {
            let patterns = [
                ("/foo//bar", "/foo/bar"),
                ("/foo///bar", "/foo/bar"),
                ("///foo///bar///", "/foo/bar/"),
                ("/foo/bar///", "/foo/bar/"),
                ("/foo/ /bar", "/foo/ /bar"),
            ];
            for (pattern, expected) in patterns.iter() {
                let regex = path_regex(pattern)?;
                assert!(regex.is_match(expected));
            }

            Ok(())
        }
    }
}
