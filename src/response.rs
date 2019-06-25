use serde::Deserialize;
use std::str::FromStr;
use std::time::SystemTime;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "usage_report")]
pub struct UsageReport {
    pub metric: String,
    pub period: String,
    pub period_start: String,
    pub period_end: String,
    pub max_value: u64,
    pub current_value: u64,
}

// Unfortunately the XML output from Apisonator includes a rather useless "usage_reports" tag that
// is then followed by a "usage_report" tag in each UsageReport, so we need to wrap that up.
#[derive(Debug, Deserialize, PartialEq)]
enum URWrapper {
    #[serde(rename = "usage_report")]
    UsageReport(UsageReport),
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "status")]
pub struct Authorization {
    authorized: String,
    plan: String,
    usage_reports: Vec<URWrapper>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct PeriodInstance {
    start: SystemTime,
    end: SystemTime,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Period {
    Minute(PeriodInstance),
    Hour(PeriodInstance),
    Day(PeriodInstance),
    Week(PeriodInstance),
    Month(PeriodInstance),
    Year(PeriodInstance),
    Eternity,
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
            </usage_reports>
        </status>
        "##;

        let parsed_auth = Authorization::from_str(s).unwrap();

        println!("Parsed: {:#?}", parsed_auth);
    }
}
