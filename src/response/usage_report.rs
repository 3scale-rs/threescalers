use std::prelude::v1::*;

use core::fmt;

use chrono::DateTime;
use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer,
};

mod systemtime {
    use chrono::{DateTime, LocalResult};

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct PeriodTime(pub i64);

    impl<Tz: chrono::TimeZone> From<LocalResult<DateTime<Tz>>> for PeriodTime {
        fn from(dt: LocalResult<DateTime<Tz>>) -> PeriodTime {
            PeriodTime(dt.single().unwrap().timestamp())
        }
    }

    impl<Tz: chrono::TimeZone> From<DateTime<Tz>> for PeriodTime {
        fn from(dt: DateTime<Tz>) -> PeriodTime {
            PeriodTime(dt.timestamp())
        }
    }
}

pub use systemtime::PeriodTime;

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UsageReportError {
    LimitsExceeded(u64),
    Overflow,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename = "usage_report")]
pub struct UsageReport {
    pub metric: String,
    pub period: Period,
    pub period_start: PeriodTime,
    pub period_end: PeriodTime,
    pub max_value: u64,
    pub current_value: u64,
}

impl UsageReport {
    pub fn metric(&self) -> &str {
        self.metric.as_ref()
    }

    pub fn period(&self) -> &Period {
        &self.period
    }

    pub fn period_times(&self) -> (&PeriodTime, &PeriodTime) {
        (&self.period_start, &self.period_end)
    }

    pub fn max_value(&self) -> u64 {
        self.max_value
    }

    pub fn current_value(&self) -> u64 {
        self.current_value
    }

    pub fn remaining(&self) -> u64 {
        self.max_value.saturating_sub(self.current_value)
    }

    pub fn is_limited(&self) -> bool {
        self.current_value >= self.max_value
    }

    pub fn authorize(&self, hits: u64) -> Result<u64, UsageReportError> {
        let new_hits = self
            .current_value
            .checked_add(hits)
            .ok_or(UsageReportError::Overflow)?;
        if new_hits > self.max_value {
            Err(UsageReportError::LimitsExceeded(new_hits - self.max_value))
        } else {
            Ok(new_hits)
        }
    }

    pub fn report(&mut self, hits: u64) -> Result<u64, UsageReportError> {
        self.current_value = self
            .current_value
            .checked_add(hits)
            .ok_or(UsageReportError::Overflow)?;

        Ok(self.current_value)
    }

    pub fn reset<V: Into<Option<u64>>>(&mut self, val: V) {
        self.current_value = val.into().unwrap_or(0);
    }
}

// Unfortunately the XML output from Apisonator includes a rather useless "usage_reports" tag that
// is then followed by a "usage_report" tag in each UsageReport, so we need to wrap that up.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum UsageReports {
    #[serde(rename = "usage_report")]
    UsageReports(Vec<UsageReport>),
}

impl UsageReports {
    pub fn as_vec(&self) -> &Vec<UsageReport> {
        match self {
            Self::UsageReports(v) => v,
        }
    }

    pub fn as_vec_mut(&mut self) -> &mut Vec<UsageReport> {
        match self {
            Self::UsageReports(v) => v,
        }
    }

    pub fn into_inner(self) -> Vec<UsageReport> {
        match self {
            Self::UsageReports(v) => v,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Period {
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
    Eternity,
    Other(String),
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
            period_name => Ok(Period::Other(period_name.into())),
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

        let ts_str = timestamp.as_str();
        let dt = DateTime::parse_from_str(ts_str, "%Y-%m-%d %H:%M:%S %z").map_err(|e| {
            de::Error::custom(format_args!(
                "invalid timestamp {}, expected %Y-%m-%d %H:%M:%S %z: {:?}",
                ts_str, e
            ))
        })?;

        Ok(PeriodTime::from(dt))
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;

    fn sample_usage_report() -> UsageReport {
        UsageReport {
            metric: "hits".into(),
            period: Period::Minute,
            period_start: Utc.with_ymd_and_hms(2021, 6, 22, 16, 58, 0).into(),
            period_end: Utc.with_ymd_and_hms(2021, 6, 22, 16, 59, 0).into(),
            max_value: 10,
            current_value: 0,
        }
    }

    #[test]
    fn test_usage_report_getters() {
        let ur = sample_usage_report();

        assert_eq!(ur.metric(), "hits");
        assert_eq!(ur.period(), &Period::Minute);
        assert_eq!(
            ur.period_times(),
            (
                &PeriodTime::from(Utc.with_ymd_and_hms(2021, 6, 22, 16, 58, 0)),
                &PeriodTime::from(Utc.with_ymd_and_hms(2021, 6, 22, 16, 59, 0))
            )
        );
        assert_eq!(ur.max_value(), 10);
        assert_eq!(ur.current_value(), 0);
    }

    #[test]
    fn test_usage_report_counters() {
        let mut ur = sample_usage_report();

        assert!(!ur.is_limited());
        let remaining = ur.remaining();
        let auth = ur.authorize(remaining);
        assert!(auth.is_ok());
        let report = ur.report(remaining);
        assert!(report.is_ok());
        let report_val = report.unwrap();
        assert_eq!(report_val, ur.current_value());
        assert_eq!(report_val, ur.max_value());
        let auth = ur.authorize(0);
        assert!(auth.is_ok());
        let auth = ur.authorize(1);
        assert!(auth.is_err());
        let exceeded_limits = auth.unwrap_err();
        assert_eq!(exceeded_limits, UsageReportError::LimitsExceeded(1));
        assert!(ur.is_limited());
        ur.reset(std::u64::MAX);
        let new_auth = ur.authorize(1);
        assert!(new_auth.is_err());
        let auth_err = new_auth.unwrap_err();
        assert_eq!(auth_err, UsageReportError::Overflow);
    }

    #[test]
    fn test_deserialization() {
        let xml = r#"
            <usage_report metric="hits" period="minute">
                <period_start>2021-06-22 16:58:00 +0000</period_start>
                <period_end>2021-06-22 16:59:00 +0000</period_end>
                <max_value>10</max_value>
                <current_value>0</current_value>
            </usage_report>
        "#;

        let ur = serde_xml_rs::from_str::<UsageReport>(xml);
        assert!(ur.is_ok());
        let ur = ur.unwrap();
        assert_eq!(ur, sample_usage_report());
    }
}
