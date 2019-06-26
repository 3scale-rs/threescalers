/// Static endpoints from the 3scale Apisonator service
///
use http_types::Method;

pub const AUTHORIZE_ENDPOINT: (Method, &str) = (Method::GET, "/transactions/authorize.xml");
pub const AUTHREP_ENDPOINT: (Method, &str) = (Method::GET, "/transactions/authrep.xml");
pub const REPORT_ENDPOINT: (Method, &str) = (Method::POST, "/transactions.xml");
pub const OAUTH_AUTHORIZE_ENDPOINT: (Method, &str) =
    (Method::GET, "/transactions/oauth_authorize.xml");
pub const OAUTH_AUTHREP_ENDPOINT: (Method, &str) = (Method::GET, "/transactions/oauth_authrep.xml");
