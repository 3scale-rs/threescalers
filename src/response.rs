use std::prelude::v1::*;

use std::str::FromStr;

use serde::Deserialize;

mod app_keys_list;
pub use app_keys_list::ListAppKeys;

mod metrics_hierarchy;
pub use metrics_hierarchy::MetricsHierarchy;

mod usage_report;
pub use usage_report::{Period, PeriodTime, UsageReport, UsageReportError, UsageReports};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Authorization {
    Status(AuthorizationStatus),
    Error(AuthorizationError),
}

impl Authorization {
    pub fn is_status(&self) -> bool {
        matches!(self, Self::Status(_))
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }

    // Note that the `into_inner` call will return a `Result` that is much
    // more like an `Either`, since the Ok variant will represent the AuthStatus
    // and the Err variant will represent the parsed AuthError, but the Err
    // variant is not an `Error` type itself.
    pub fn into_inner(self) -> Result<AuthorizationStatus, AuthorizationError> {
        match self {
            Self::Status(st) => Ok(st),
            Self::Error(err) => Err(err),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AuthorizationStatus {
    authorized: bool,
    reason: Option<String>,
    plan: String,
    usage_reports: Option<UsageReports>,

    #[serde(rename = "hierarchy")]
    metrics_hierarchy: Option<MetricsHierarchy>,

    #[serde(rename = "app_keys")]
    app_keys: Option<ListAppKeys>,
}

impl AuthorizationStatus {
    pub fn is_authorized(&self) -> bool {
        self.authorized
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    pub fn authorized(&self) -> Result<(), &str> {
        if self.authorized {
            Ok(())
        } else {
            Err(self.reason.as_deref().unwrap_or("unspecified reason"))
        }
    }

    pub fn plan(&self) -> &str {
        self.plan.as_ref()
    }

    pub fn app_keys(&self) -> Option<&ListAppKeys> {
        self.app_keys.as_ref()
    }

    pub fn usage_reports(&self) -> Option<&Vec<UsageReport>> {
        self.usage_reports.as_ref().map(|ur| ur.as_vec())
    }

    pub fn usage_reports_mut(&mut self) -> Option<&mut Vec<UsageReport>> {
        self.usage_reports.as_mut().map(|ur| ur.as_vec_mut())
    }

    pub fn hierarchy(&self) -> Option<&MetricsHierarchy> {
        self.metrics_hierarchy.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AuthorizationError {
    code: String,
    #[serde(rename = "$value")]
    description: String,
}

impl AuthorizationError {
    pub fn code(&self) -> &str {
        self.code.as_ref()
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }
}

#[repr(transparent)]
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
    use super::{UsageReports::UsageReports, *};
    use chrono::prelude::*;

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

        let usage_reports = vec![
            UsageReport {
                metric: String::from("products"),
                period: Period::Minute,
                period_start: Utc.ymd(2019, 6, 5).and_hms(16, 24, 0).into(),
                period_end: Utc.ymd(2019, 6, 5).and_hms(16, 25, 0).into(),
                max_value: 5,
                current_value: 0,
            },
            UsageReport {
                metric: String::from("products"),
                period: Period::Month,
                period_start: Utc.ymd(2019, 6, 1).and_hms(0, 0, 0).into(),
                period_end: Utc.ymd(2019, 7, 1).and_hms(0, 0, 0).into(),
                max_value: 50,
                current_value: 0,
            },
        ];
        let app_plan = "App Plan";

        let expected_auth = Authorization::Status(AuthorizationStatus {
            authorized: true,
            reason: None,
            plan: String::from(app_plan),
            metrics_hierarchy: None,
            app_keys: None,
            usage_reports: Some(UsageReports(usage_reports.clone())),
        });

        assert!(parsed_auth.is_status());
        assert_eq!(parsed_auth, expected_auth);
        let inner = parsed_auth.into_inner();
        assert_eq!(inner, expected_auth.into_inner());
        assert!(inner.is_ok());

        let auth_status = inner.unwrap();
        assert!(auth_status.is_authorized());
        assert!(auth_status.reason().is_none());
        assert_eq!(auth_status.plan(), app_plan);
        assert!(auth_status.hierarchy().is_none());
        assert!(auth_status.app_keys().is_none());
        let ur = auth_status.usage_reports();
        assert_eq!(ur, Some(&usage_reports));
        let ur = ur.unwrap();
        assert_eq!(ur, &usage_reports);
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
    fn parse_response_with_no_usage_reports() {
        let s = r##"
        <?xml version="1.0" encoding="UTF-8"?>
        <status>
            <authorized>true</authorized>
            <plan>App Plan</plan>
        </status>
        "##;
        let expected_auth = Authorization::Status(AuthorizationStatus {
            authorized: true,
            reason: None,
            plan: "App Plan".into(),
            usage_reports: None,
            metrics_hierarchy: None,
            app_keys: None,
        });
        let parsed_auth = Authorization::from_str(s)
            .expect("failed to parse authorization without usage reports");

        assert!(parsed_auth.is_status());
        assert_eq!(parsed_auth, expected_auth);
        let inner = parsed_auth.into_inner();
        assert_eq!(inner, expected_auth.into_inner());
        assert!(inner.is_ok());

        let auth_status = inner.unwrap();
        assert!(auth_status.is_authorized());
        assert!(auth_status.usage_reports().is_none());
    }

    #[test]
    fn parse_error_authorization() {
        let xml_response = r##"
        <?xml version="1.0" encoding="UTF-8"?>
        <error code="user_key_invalid">user key "some_user_key" is invalid</error>
        "##;

        let parsed_auth = Authorization::from_str(xml_response).unwrap();

        let expected_auth = Authorization::Error(AuthorizationError {
            code: String::from("user_key_invalid"),
            description: r#"user key "some_user_key" is invalid"#.into(),
        });

        assert!(parsed_auth.is_error());
        assert_eq!(parsed_auth, expected_auth);
        let inner = parsed_auth.into_inner();
        assert_eq!(inner, expected_auth.into_inner());
        assert!(inner.is_err());

        let auth_error = inner.unwrap_err();
        assert_eq!(auth_error.code(), "user_key_invalid");
        assert_eq!(
            auth_error.description(),
            r#"user key "some_user_key" is invalid"#
        );
    }

    #[test]
    fn parse_denied_authorization() {
        let xml_response = r##"
        <?xml version="1.0" encoding="UTF-8"?>
        <status>
          <authorized>false</authorized>
          <reason>application key is missing</reason>
          <plan>sample</plan>
          <usage_reports>
            <usage_report metric="ticks" period="minute">
              <period_start>2021-06-08 18:07:00 +0000</period_start>
              <period_end>2021-06-08 18:08:00 +0000</period_end>
              <max_value>5</max_value>
              <current_value>0</current_value>
            </usage_report>
          </usage_reports>
        </status>
        "##;

        let parsed_auth = Authorization::from_str(xml_response).unwrap();

        let reason = "application key is missing";
        let app_plan = "sample";
        let usage_reports = vec![UsageReport {
            metric: String::from("ticks"),
            period: Period::Minute,
            period_start: Utc.ymd(2021, 6, 8).and_hms(18, 7, 0).into(),
            period_end: Utc.ymd(2021, 6, 8).and_hms(18, 8, 0).into(),
            max_value: 5,
            current_value: 0,
        }];

        let expected_auth = Authorization::Status(AuthorizationStatus {
            authorized: false,
            reason: Some(reason.into()),
            plan: app_plan.into(),
            usage_reports: Some(UsageReports(usage_reports.clone())),
            metrics_hierarchy: None,
            app_keys: None,
        });

        assert!(parsed_auth.is_status());
        assert_eq!(parsed_auth, expected_auth);
        let inner = parsed_auth.into_inner();
        assert_eq!(inner, expected_auth.into_inner());
        assert!(inner.is_ok());

        let auth_status = inner.unwrap();
        assert!(!auth_status.is_authorized());
        assert_eq!(auth_status.reason(), Some(reason));
        assert_eq!(auth_status.plan(), app_plan);
        assert!(auth_status.hierarchy().is_none());
        assert!(auth_status.app_keys().is_none());
        assert_eq!(auth_status.usage_reports(), Some(&usage_reports));
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
            "parent1",
            vec![String::from("child1"), String::from("child2")],
        );
        expected_hierarchy.insert("parent2", vec![String::from("child3")]);

        let expected_auth = Authorization::Status(AuthorizationStatus {
            authorized: true,
            reason: None,
            plan: String::from("Basic"),
            metrics_hierarchy: Some(expected_hierarchy),
            app_keys: None,
            usage_reports: Some(UsageReports(vec![
                UsageReport {
                    metric: String::from("parent1"),
                    period: Period::Day,
                    period_start: Utc.ymd(2016, 1, 1).and_hms(0, 0, 0).into(),
                    period_end: Utc.ymd(2016, 1, 2).and_hms(0, 0, 0).into(),
                    max_value: 100,
                    current_value: 20,
                },
                UsageReport {
                    metric: String::from("parent2"),
                    period: Period::Day,
                    period_start: Utc.ymd(2016, 1, 1).and_hms(0, 0, 0).into(),
                    period_end: Utc.ymd(2016, 1, 2).and_hms(0, 0, 0).into(),
                    max_value: 100,
                    current_value: 10,
                },
                UsageReport {
                    metric: String::from("child1"),
                    period: Period::Day,
                    period_start: Utc.ymd(2016, 1, 1).and_hms(0, 0, 0).into(),
                    period_end: Utc.ymd(2016, 1, 2).and_hms(0, 0, 0).into(),
                    max_value: 100,
                    current_value: 10,
                },
                UsageReport {
                    metric: String::from("child2"),
                    period: Period::Day,
                    period_start: Utc.ymd(2016, 1, 1).and_hms(0, 0, 0).into(),
                    period_end: Utc.ymd(2016, 1, 2).and_hms(0, 0, 0).into(),
                    max_value: 100,
                    current_value: 10,
                },
                UsageReport {
                    metric: String::from("child3"),
                    period: Period::Day,
                    period_start: Utc.ymd(2016, 1, 1).and_hms(0, 0, 0).into(),
                    period_end: Utc.ymd(2016, 1, 2).and_hms(0, 0, 0).into(),
                    max_value: 100,
                    current_value: 10,
                },
            ])),
        });

        assert_eq!(parsed_auth, expected_auth);
    }

    #[test]
    fn metrics_hierarchy_remove() {
        let mut hierarchy = MetricsHierarchy::new();

        hierarchy.insert(
            "parent1",
            vec![String::from("child1"), String::from("child2")],
        );
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

        hierarchy.insert(
            a_parent,
            vec![String::from("child1"), String::from("child2")],
        );
        hierarchy.insert("parent2", vec![String::from("child3")]);

        assert_eq!(hierarchy.parent_of("child2"), Some(a_parent));
        assert_eq!(hierarchy.parent_of("child3"), Some("parent2"));
        assert_eq!(hierarchy.parent_of("nonchild"), None);
        assert_eq!(hierarchy.parent_of(a_parent), None);
    }

    #[test]
    fn parse_app_keys() {
        let xml_response = r##"
        <?xml version="1.0" encoding="UTF-8"?>
        <status>
            <authorized>true</authorized>
            <plan>Basic</plan>
            <app_keys app="app_id" svc="service_id">
                <key id="a_secret_key" />
                <key id="another_secret_key"/>
            </app_keys>
        </status>
        "##;

        let parsed_auth = Authorization::from_str(xml_response).unwrap();

        let expected_app_keys = ListAppKeys::new(
            "service_id".into(),
            "app_id".into(),
            vec!["a_secret_key", "another_secret_key"],
        );

        let expected_auth = Authorization::Status(AuthorizationStatus {
            authorized: true,
            reason: None,
            plan: String::from("Basic"),
            app_keys: Some(expected_app_keys),
            metrics_hierarchy: None,
            usage_reports: None,
        });

        assert_eq!(parsed_auth, expected_auth);
    }

    #[test]
    fn parse_empty_app_keys() {
        let xml_response = r##"
        <?xml version="1.0" encoding="UTF-8"?>
        <status>
            <authorized>true</authorized>
            <plan>Basic</plan>
            <app_keys app="app_id" svc="service_id">
            </app_keys>
        </status>
        "##;

        let parsed_auth = Authorization::from_str(xml_response).unwrap();

        let expected_app_keys = ListAppKeys::new(
            "service_id".into(),
            "app_id".into(),
            core::iter::empty::<crate::application::AppKey>(),
        );

        let expected_auth = Authorization::Status(AuthorizationStatus {
            authorized: true,
            reason: None,
            plan: String::from("Basic"),
            app_keys: Some(expected_app_keys),
            metrics_hierarchy: None,
            usage_reports: None,
        });

        assert_eq!(parsed_auth, expected_auth);
    }
}
