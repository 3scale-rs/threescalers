use std::prelude::v1::*;

use chrono::prelude::*;
use serde::{
    de::{
        self,
        Deserializer,
        MapAccess,
        Visitor,
    },
    Deserialize,
};
use std::{
    collections::{
        btree_map::{
            Iter,
            IterMut,
        },
        BTreeMap,
    },
    fmt,
    str::FromStr,
};

mod systemtime {
    use chrono::DateTime;

    #[repr(transparent)]
    #[derive(Debug, PartialEq)]
    pub struct PeriodTime(pub i64);

    impl<Tz: chrono::TimeZone> From<DateTime<Tz>> for PeriodTime {
        fn from(dt: DateTime<Tz>) -> PeriodTime {
            PeriodTime(dt.timestamp())
        }
    }
}

pub use systemtime::PeriodTime;

// We might want to consider moving from a BTreeMap to a Vec, as most of the time this btreemap will
// contain a (very) small number of entries.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MetricsHierarchy(BTreeMap<String, Vec<String>>);

impl MetricsHierarchy {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert<S: Into<String>, V: Into<Vec<String>>>(&mut self,
                                                         parent_metric: S,
                                                         children_metrics: V)
                                                         -> Option<Vec<String>> {
        self.0.insert(parent_metric.into(), children_metrics.into())
    }

    pub fn remove<S: AsRef<str>>(&mut self, parent_metric: S) -> Option<Vec<String>> {
        self.0.remove(parent_metric.as_ref())
    }

    pub fn iter(&self) -> Iter<String, Vec<String>> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<String, Vec<String>> {
        self.0.iter_mut()
    }

    pub fn into_inner(self) -> BTreeMap<String, Vec<String>> {
        self.0
    }

    /// Retrieves the parent metric of a given metric. Note that Apisonator metrics have 0 or
    /// 1 parent metrics, not multiple.
    pub fn parent_of(&self, metric_name: &str) -> Option<&str> {
        self.iter().find_map(|(parent, v)| {
                       if v.iter().any(|child| metric_name == child) {
                           Some(parent.as_str())
                       } else {
                           None
                       }
                   })
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "usage_report")]
pub struct UsageReport {
    pub metric:        String,
    pub period:        Period,
    pub period_start:  PeriodTime,
    pub period_end:    PeriodTime,
    pub max_value:     u64,
    pub current_value: u64,
}

// Unfortunately the XML output from Apisonator includes a rather useless "usage_reports" tag that
// is then followed by a "usage_report" tag in each UsageReport, so we need to wrap that up.
#[derive(Debug, Deserialize, PartialEq)]
pub enum UsageReports {
    #[serde(rename = "usage_report")]
    UsageReports(Vec<UsageReport>),
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Authorization {
    #[serde(rename = "status")]
    Ok(OkAuthorization),

    #[serde(rename = "error")]
    Denied(DeniedAuthorization),
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct OkAuthorization {
    plan:          String,
    usage_reports: UsageReports,

    #[serde(rename = "hierarchy")]
    metrics_hierarchy: Option<MetricsHierarchy>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DeniedAuthorization {
    code: String,
}

#[derive(Debug, PartialEq)]
pub enum Period {
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
    Eternity,
}

struct PeriodStringVisitor;

impl<'de> Visitor<'de> for PeriodStringVisitor {
    type Value = Period;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string that represents a period")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where E: de::Error
    {
        match v {
            "minute" => Ok(Period::Minute),
            "hour" => Ok(Period::Hour),
            "day" => Ok(Period::Day),
            "week" => Ok(Period::Week),
            "month" => Ok(Period::Month),
            "year" => Ok(Period::Year),
            "eternity" => Ok(Period::Eternity),
            _ => Err(E::custom("Invalid period")),
        }
    }
}

impl<'de> Deserialize<'de> for Period {
    fn deserialize<D>(deserializer: D) -> Result<Period, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_any(PeriodStringVisitor)
    }
}

struct TimestampVisitor;

impl<'de> Visitor<'de> for TimestampVisitor {
    type Value = PeriodTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string that represents a timestamp")
    }

    fn visit_map<V>(self, mut map: V) -> Result<PeriodTime, V::Error>
        where V: MapAccess<'de>
    {
        // We know there's only one key with one value.
        // The key is not used, but we need to call "next_key()". From the
        // docs: "Calling `next_value` before `next_key` is incorrect and is
        // allowed to panic or return bogus results.".
        let _key: Option<String> = map.next_key()?;
        let timestamp: String = map.next_value()?;

        let ts_str = timestamp.as_str();
        let dt =
            DateTime::parse_from_str(ts_str, "%Y-%m-%d %H:%M:%S %z").map_err(|e| {
                de::Error::custom(format_args!("invalid timestamp {}, expected %Y-%m-%d %H:%M:%S %z: {:?}",
                                               ts_str, e))
            })?;

        Ok(PeriodTime::from(dt))
    }
}

impl<'de> Deserialize<'de> for PeriodTime {
    fn deserialize<D>(deserializer: D) -> Result<PeriodTime, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_any(TimestampVisitor)
    }
}

struct MetricsHierarchyVisitor;

impl<'de> Visitor<'de> for MetricsHierarchyVisitor {
    type Value = MetricsHierarchy;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a structure that represents a hierarchy of metrics")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
        where V: MapAccess<'de>
    {
        let mut hierarchy = MetricsHierarchy::new();

        // The key in the hierarchy structure is always "metric". It is not
        // used, but we need to read it to get the value.
        while map.next_key::<String>()?.is_some() {
            let val: BTreeMap<String, String> = map.next_value()?;

            let parent_metric = val["name"].to_owned();
            let children_metrics = val["children"].split(' ')
                                                  .map(|s| s.to_owned())
                                                  .collect::<Vec<_>>();

            hierarchy.insert(parent_metric, children_metrics);
        }

        Ok(hierarchy)
    }
}

impl<'de> Deserialize<'de> for MetricsHierarchy {
    fn deserialize<D>(deserializer: D) -> Result<MetricsHierarchy, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_any(MetricsHierarchyVisitor)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct UsageData {
    max_value:     u64,
    current_value: u64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Metric(pub String);

impl FromStr for Authorization {
    type Err = serde_xml_rs::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_xml_rs::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        UsageReports::*,
        *,
    };

    #[test]
    fn parse() {
        let s = r##"
        <?xml version="1.0" encoding="UTF-8"?>
        <status>
            <authorized>true</authorized>
            <plan>App Plan</plan>
            <usage_reports>
                <usage_report metric="products" period="minute">
                    <period_start>2019-06-05 16:24:00 +0000</period_start>
                    <period_end>2019-06-05 16:25:00 +0000</period_end>
                    <max_value>5</max_value>
                    <current_value>0</current_value>
                </usage_report>
                <usage_report metric="products" period="month">
                    <period_start>2019-06-01 00:00:00 +0000</period_start>
                    <period_end>2019-07-01 00:00:00 +0000</period_end>
                    <max_value>50</max_value>
                    <current_value>0</current_value>
                </usage_report>
            </usage_reports>
        </status>
        "##;

        let parsed_auth = Authorization::from_str(s).unwrap();

        let expected_auth = Authorization::Ok(OkAuthorization { plan:              String::from("App Plan"),
                                                                metrics_hierarchy: None,
                                                                usage_reports:     UsageReports(vec![
                UsageReport {
                    metric: String::from("products"),
                    period: Period::Minute,
                    period_start: DateTime::<Utc>::from(Utc.ymd(2019, 6, 5).and_hms(16, 24, 0)).into(),
                    period_end: DateTime::<Utc>::from(Utc.ymd(2019, 6, 5).and_hms(16, 25, 0)).into(),
                    max_value: 5,
                    current_value: 0,
                },
                UsageReport {
                    metric: String::from("products"),
                    period: Period::Month,
                    period_start: DateTime::<Utc>::from(Utc.ymd(2019, 6, 1).and_hms(0, 0, 0)).into(),
                    period_end: DateTime::<Utc>::from(Utc.ymd(2019, 7, 1).and_hms(0, 0, 0)).into(),
                    max_value: 50,
                    current_value: 0,
                },
            ]), });

        assert_eq!(parsed_auth, expected_auth);
    }

    #[test]
    fn parse_invalid_date_format() {
        let s = r##"
        <?xml version="1.0" encoding="UTF-8"?>
        <status>
            <authorized>true</authorized>
            <plan>App Plan</plan>
            <usage_reports>
                <usage_report metric="products" period="minute">
                    <period_start>05-06-2019 16:24:00 +0000</period_start>
                    <period_end>05-06-2019 16:25:00 +0000</period_end>
                    <max_value>5</max_value>
                    <current_value>0</current_value>
                </usage_report>
                <usage_report metric="products" period="month">
                    <period_start>2019-06-01 00:00:00 +0000</period_start>
                    <period_end>2019-07-01 00:00:00 +0000</period_end>
                    <max_value>50</max_value>
                    <current_value>0</current_value>
                </usage_report>
            </usage_reports>
        </status>
        "##;

        let parsed_auth = Authorization::from_str(s);

        assert!(parsed_auth.is_err());

        let s = format!("{}", parsed_auth.unwrap_err());
        assert!(s.contains("invalid timestamp"));
    }

    #[test]
    fn parse_denied_authorization() {
        let xml_response = r##"
        <?xml version="1.0" encoding="UTF-8"?>
        <error code="user_key_invalid">user key "some_user_key" is invalid</error>
        "##;

        let parsed_auth = Authorization::from_str(xml_response).unwrap();

        let expected_auth =
            Authorization::Denied(DeniedAuthorization { code: String::from("user_key_invalid"), });
        assert_eq!(parsed_auth, expected_auth);
    }

    #[test]
    fn parse_metrics_hierarchy() {
        let xml_response = r##"
        <?xml version="1.0" encoding="UTF-8"?>
        <status>
            <authorized>true</authorized>
            <plan>Basic</plan>
            <usage_reports>
                <usage_report metric="parent1" period="day">
                    <period_start>2016-01-01 00:00:00 +0000</period_start>
                    <period_end>2016-01-02 00:00:00 +0000</period_end>
                    <max_value>100</max_value>
                    <current_value>20</current_value>
                </usage_report>
                <usage_report metric="parent2" period="day">
                    <period_start>2016-01-01 00:00:00 +0000</period_start>
                    <period_end>2016-01-02 00:00:00 +0000</period_end>
                    <max_value>100</max_value>
                    <current_value>10</current_value>
                </usage_report>
                <usage_report metric="child1" period="day">
                    <period_start>2016-01-01 00:00:00 +0000</period_start>
                    <period_end>2016-01-02 00:00:00 +0000</period_end>
                    <max_value>100</max_value>
                    <current_value>10</current_value>
                </usage_report>
                <usage_report metric="child2" period="day">
                    <period_start>2016-01-01 00:00:00 +0000</period_start>
                    <period_end>2016-01-02 00:00:00 +0000</period_end>
                    <max_value>100</max_value>
                    <current_value>10</current_value>
                </usage_report>
                <usage_report metric="child3" period="day">
                    <period_start>2016-01-01 00:00:00 +0000</period_start>
                    <period_end>2016-01-02 00:00:00 +0000</period_end>
                    <max_value>100</max_value>
                    <current_value>10</current_value>
                </usage_report>
            </usage_reports>
            <hierarchy>
                <metric name="parent1" children="child1 child2" />
                <metric name="parent2" children="child3" />
            </hierarchy>
        </status>
        "##;

        let parsed_auth = Authorization::from_str(xml_response).unwrap();

        let mut expected_hierarchy = MetricsHierarchy::new();
        expected_hierarchy.insert("parent1", vec![String::from("child1"), String::from("child2")]);
        expected_hierarchy.insert("parent2", vec![String::from("child3")]);

        let expected_auth = Authorization::Ok(OkAuthorization { plan:              String::from("Basic"),
                                                                metrics_hierarchy: Some(expected_hierarchy),
                                                                usage_reports:     UsageReports(vec![
                UsageReport {
                    metric: String::from("parent1"),
                    period: Period::Day,
                    period_start: DateTime::<Utc>::from(Utc.ymd(2016, 1, 1).and_hms(0, 0, 0)).into(),
                    period_end: DateTime::<Utc>::from(Utc.ymd(2016, 1, 2).and_hms(0, 0, 0)).into(),
                    max_value: 100,
                    current_value: 20,
                },
                UsageReport {
                    metric: String::from("parent2"),
                    period: Period::Day,
                    period_start: DateTime::<Utc>::from(Utc.ymd(2016, 1, 1).and_hms(0, 0, 0)).into(),
                    period_end: DateTime::<Utc>::from(Utc.ymd(2016, 1, 2).and_hms(0, 0, 0)).into(),
                    max_value: 100,
                    current_value: 10,
                },
                UsageReport {
                    metric: String::from("child1"),
                    period: Period::Day,
                    period_start: DateTime::<Utc>::from(Utc.ymd(2016, 1, 1).and_hms(0, 0, 0)).into(),
                    period_end: DateTime::<Utc>::from(Utc.ymd(2016, 1, 2).and_hms(0, 0, 0)).into(),
                    max_value: 100,
                    current_value: 10,
                },
                UsageReport {
                    metric: String::from("child2"),
                    period: Period::Day,
                    period_start: DateTime::<Utc>::from(Utc.ymd(2016, 1, 1).and_hms(0, 0, 0)).into(),
                    period_end: DateTime::<Utc>::from(Utc.ymd(2016, 1, 2).and_hms(0, 0, 0)).into(),
                    max_value: 100,
                    current_value: 10,
                },
                UsageReport {
                    metric: String::from("child3"),
                    period: Period::Day,
                    period_start: DateTime::<Utc>::from(Utc.ymd(2016, 1, 1).and_hms(0, 0, 0)).into(),
                    period_end: DateTime::<Utc>::from(Utc.ymd(2016, 1, 2).and_hms(0, 0, 0)).into(),
                    max_value: 100,
                    current_value: 10,
                },
            ]), });

        assert_eq!(parsed_auth, expected_auth);
    }

    #[test]
    fn metrics_hierarchy_remove() {
        let mut hierarchy = MetricsHierarchy::new();

        hierarchy.insert("parent1", vec![String::from("child1"), String::from("child2")]);
        hierarchy.insert("parent2", vec![String::from("child3")]);

        hierarchy.remove("parent1");

        let mut expected_hierarchy = MetricsHierarchy::new();

        expected_hierarchy.insert("parent2", vec![String::from("child3")]);

        assert_eq!(hierarchy, expected_hierarchy);
    }

    #[test]
    fn metrics_hierarchy_parent_of() {
        let a_parent = "a_parent";
        let mut hierarchy = MetricsHierarchy::new();

        hierarchy.insert(a_parent, vec![String::from("child1"), String::from("child2")]);
        hierarchy.insert("parent2", vec![String::from("child3")]);

        assert_eq!(hierarchy.parent_of("child2"), Some(a_parent));
        assert_eq!(hierarchy.parent_of("child3"), Some("parent2"));
        assert_eq!(hierarchy.parent_of("nonchild"), None);
        assert_eq!(hierarchy.parent_of(a_parent), None);
    }
}
