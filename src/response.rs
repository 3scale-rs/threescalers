use chrono::prelude::*;
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::time::SystemTime;

#[derive(Debug, PartialEq)]
pub struct PeriodTime(SystemTime);

impl From<SystemTime> for PeriodTime {
    fn from(st: SystemTime) -> Self {
        PeriodTime(st)
    }
}

impl From<PeriodTime> for SystemTime {
    fn from(pt: PeriodTime) -> Self {
        pt.0
    }
}

#[derive(Debug, PartialEq)]
pub struct MetricsHierarchy(HashMap<String, Vec<String>>);

impl MetricsHierarchy {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, parent_metric: String, children_metrics: Vec<String>) {
        self.0.insert(parent_metric, children_metrics);
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "usage_report")]
pub struct UsageReport {
    pub metric: String,
    pub period: Period,
    pub period_start: PeriodTime,
    pub period_end: PeriodTime,
    pub max_value: u64,
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
#[serde(rename = "status")]
pub struct Authorization {
    authorized: bool,
    plan: String,
    usage_reports: UsageReports,

    #[serde(rename = "hierarchy")]
    metrics_hierarchy: Option<MetricsHierarchy>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct PeriodInstance {
    start: SystemTime,
    end: SystemTime,
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
    where
        E: de::Error,
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
    where
        D: Deserializer<'de>,
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
    where
        V: MapAccess<'de>,
    {
        // We know there's only one key with one value.
        // The key is not used, but we need to call "next_key()". From the
        // docs: "Calling `next_value` before `next_key` is incorrect and is
        // allowed to panic or return bogus results.".
        let _key: Option<String> = map.next_key()?;
        let timestamp: String = map.next_value()?;

        let dt = DateTime::parse_from_str(timestamp.as_str(), "%Y-%m-%d %H:%M:%S %z").unwrap();

        Ok(PeriodTime(dt.into()))
    }
}

impl<'de> Deserialize<'de> for PeriodTime {
    fn deserialize<D>(deserializer: D) -> Result<PeriodTime, D::Error>
    where
        D: Deserializer<'de>,
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
    where
        V: MapAccess<'de>,
    {
        let mut hierarchy = MetricsHierarchy::new();

        // The key in the hierarchy structure is always "metric". It is not
        // used, but we need to read it to get the value.
        while let Some(_) = map.next_key::<String>()? {
            let val: HashMap<String, String> = map.next_value()?;

            let parent_metric = val["name"].to_owned();
            let children_metrics = val["children"].split(' ').map(|s| s.to_owned()).collect();

            hierarchy.insert(parent_metric, children_metrics);
        }

        Ok(hierarchy)
    }
}

impl<'de> Deserialize<'de> for MetricsHierarchy {
    fn deserialize<D>(deserializer: D) -> Result<MetricsHierarchy, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(MetricsHierarchyVisitor)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct UsageData {
    max_value: u64,
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
    use super::*;

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

        let expected_auth = Authorization {
            authorized: true,
            plan: String::from("App Plan"),
            metrics_hierarchy: None,
            usage_reports: UsageReports::UsageReports(vec![
                UsageReport {
                    metric: String::from("products"),
                    period: Period::Minute,
                    period_start: PeriodTime(Utc.ymd(2019, 6, 5).and_hms(16, 24, 0).into()),
                    period_end: PeriodTime(Utc.ymd(2019, 6, 5).and_hms(16, 25, 0).into()),
                    max_value: 5,
                    current_value: 0,
                },
                UsageReport {
                    metric: String::from("products"),
                    period: Period::Month,
                    period_start: PeriodTime(Utc.ymd(2019, 6, 1).and_hms(0, 0, 0).into()),
                    period_end: PeriodTime(Utc.ymd(2019, 7, 1).and_hms(0, 0, 0).into()),
                    max_value: 50,
                    current_value: 0,
                },
            ]),
        };

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
        expected_hierarchy.insert(
            String::from("parent1"),
            vec![String::from("child1"), String::from("child2")],
        );
        expected_hierarchy.insert(String::from("parent2"), vec![String::from("child3")]);

        let expected_auth = Authorization {
            authorized: true,
            plan: String::from("Basic"),
            metrics_hierarchy: Some(expected_hierarchy),
            usage_reports: UsageReports::UsageReports(vec![
                UsageReport {
                    metric: String::from("parent1"),
                    period: Period::Day,
                    period_start: PeriodTime(Utc.ymd(2016, 1, 1).and_hms(0, 0, 0).into()),
                    period_end: PeriodTime(Utc.ymd(2016, 1, 2).and_hms(0, 0, 0).into()),
                    max_value: 100,
                    current_value: 20,
                },
                UsageReport {
                    metric: String::from("parent2"),
                    period: Period::Day,
                    period_start: PeriodTime(Utc.ymd(2016, 1, 1).and_hms(0, 0, 0).into()),
                    period_end: PeriodTime(Utc.ymd(2016, 1, 2).and_hms(0, 0, 0).into()),
                    max_value: 100,
                    current_value: 10,
                },
                UsageReport {
                    metric: String::from("child1"),
                    period: Period::Day,
                    period_start: PeriodTime(Utc.ymd(2016, 1, 1).and_hms(0, 0, 0).into()),
                    period_end: PeriodTime(Utc.ymd(2016, 1, 2).and_hms(0, 0, 0).into()),
                    max_value: 100,
                    current_value: 10,
                },
                UsageReport {
                    metric: String::from("child2"),
                    period: Period::Day,
                    period_start: PeriodTime(Utc.ymd(2016, 1, 1).and_hms(0, 0, 0).into()),
                    period_end: PeriodTime(Utc.ymd(2016, 1, 2).and_hms(0, 0, 0).into()),
                    max_value: 100,
                    current_value: 10,
                },
                UsageReport {
                    metric: String::from("child3"),
                    period: Period::Day,
                    period_start: PeriodTime(Utc.ymd(2016, 1, 1).and_hms(0, 0, 0).into()),
                    period_end: PeriodTime(Utc.ymd(2016, 1, 2).and_hms(0, 0, 0).into()),
                    max_value: 100,
                    current_value: 10,
                },
            ]),
        };

        assert_eq!(parsed_auth, expected_auth);
    }
}
