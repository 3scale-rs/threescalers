extern crate threescalers;

use threescalers::request::*;
use threescalers::apicall::*;
use threescalers::service::*;
use threescalers::application::*;
use threescalers::user::*;

use std::collections::HashMap;

#[test]
fn returns_auth_request_from_service_id_pkey_and_app_id() {
    let provider_key = "a_provider_key";
    let creds = Credentials::from_key(provider_key);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let app_id = "an_app_id";
    let app = Application::from_app_id(app_id);
    let call = Info::new(Type::Authorize, &service, &app, None);

    let result = call.to_request();
    let (res_endpoint, res_params) = parse_path(result.path());

    let mut expected_params = HashMap::new();
    expected_params.insert("provider_key", provider_key);
    expected_params.insert("service_id", service_id);
    expected_params.insert("app_id", app_id);

    assert_eq!("GET", result.method());
    assert_eq!("/transactions/authorize.xml", res_endpoint);
    assert_eq!(expected_params, res_params);
}

#[test]
fn returns_auth_request_from_service_id_pkey_app_id_and_user_id() {
    let provider_key = "a_provider_key";
    let creds = Credentials::from_key(provider_key);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let app_id = "an_app_id";
    let app = Application::from_app_id(app_id);
    let user_id = "a_user_id";
    let user = User::from_user_id(user_id);
    let call = Info::new(Type::Authorize, &service, &app, Some(&user));

    let result = call.to_request();
    let (res_endpoint, res_params) = parse_path(result.path());

    let mut expected_params = HashMap::new();
    expected_params.insert("provider_key", provider_key);
    expected_params.insert("service_id", service_id);
    expected_params.insert("app_id", app_id);
    expected_params.insert("user_id", user_id);

    assert_eq!("GET", result.method());
    assert_eq!("/transactions/authorize.xml", res_endpoint);
    assert_eq!(expected_params, res_params);
}

#[test]
fn returns_auth_request_from_service_id_pkey_app_id_and_oauth_user() {
    let provider_key = "a_provider_key";
    let creds = Credentials::from_key(provider_key);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let app_id = "an_app_id";
    let app = Application::from_app_id(app_id);
    let user_oauth_token = "a_user_token";
    let user = User::from_oauth_token(user_oauth_token);
    let call = Info::new(Type::Authorize, &service, &app, Some(&user));

    let result = call.to_request();
    let (res_endpoint, res_params) = parse_path(result.path());

    let mut expected_params = HashMap::new();
    expected_params.insert("provider_key", provider_key);
    expected_params.insert("service_id", service_id);
    expected_params.insert("app_id", app_id);
    expected_params.insert("access_token", user_oauth_token);

    assert_eq!("GET", result.method());
    assert_eq!("/transactions/oauth_authorize.xml", res_endpoint);
    assert_eq!(expected_params, res_params);
}

#[test]
fn returns_auth_request_from_service_id_pkey_app_id_and_app_key() {
    let provider_key = "a_provider_key";
    let creds = Credentials::from_key(provider_key);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let app_id = "an_app_id";
    let app_key = "an_app_key";
    let app = Application::from_app_id_and_key(app_id, app_key);
    let call = Info::new(Type::Authorize, &service, &app, None);

    let result = call.to_request();
    let (res_endpoint, res_params) = parse_path(result.path());

    let mut expected_params = HashMap::new();
    expected_params.insert("provider_key", provider_key);
    expected_params.insert("service_id", service_id);
    expected_params.insert("app_id", app_id);
    expected_params.insert("app_key", app_key);

    assert_eq!("GET", result.method());
    assert_eq!("/transactions/authorize.xml", res_endpoint);
    assert_eq!(expected_params, res_params);
}

#[test]
fn returns_auth_request_from_service_id_pkey_app_id_app_key_and_user_id() {
    let provider_key = "a_provider_key";
    let creds = Credentials::from_key(provider_key);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let app_id = "an_app_id";
    let app_key = "an_app_key";
    let app = Application::from_app_id_and_key(app_id, app_key);
    let user_id = "a_user_id";
    let user = User::from_user_id(user_id);
    let call = Info::new(Type::Authorize, &service, &app, Some(&user));

    let result = call.to_request();
    let (res_endpoint, res_params) = parse_path(result.path());

    let mut expected_params = HashMap::new();
    expected_params.insert("provider_key", provider_key);
    expected_params.insert("service_id", service_id);
    expected_params.insert("app_id", app_id);
    expected_params.insert("app_key", app_key);
    expected_params.insert("user_id", user_id);

    assert_eq!("GET", result.method());
    assert_eq!("/transactions/authorize.xml", res_endpoint);
    assert_eq!(expected_params, res_params);
}

#[test]
fn returns_auth_request_from_service_id_pkey_app_id_app_key_and_oauth_user() {
    let provider_key = "a_provider_key";
    let creds = Credentials::from_key(provider_key);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let app_id = "an_app_id";
    let app_key = "an_app_key";
    let app = Application::from_app_id_and_key(app_id, app_key);
    let user_oauth_token = "a_user_token";
    let user = User::from_oauth_token(user_oauth_token);
    let call = Info::new(Type::Authorize, &service, &app, Some(&user));

    let result = call.to_request();
    let (res_endpoint, res_params) = parse_path(result.path());

    let mut expected_params = HashMap::new();
    expected_params.insert("provider_key", provider_key);
    expected_params.insert("service_id", service_id);
    expected_params.insert("app_id", app_id);
    expected_params.insert("app_key", app_key);
    expected_params.insert("access_token", user_oauth_token);

    assert_eq!("GET", result.method());
    assert_eq!("/transactions/oauth_authorize.xml", res_endpoint);
    assert_eq!(expected_params, res_params);
}

#[test]
fn returns_auth_request_from_service_id_pkey_and_user_key() {
    let provider_key = "a_provider_key";
    let creds = Credentials::from_key(provider_key);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let user_key = "a_user_key";
    let app = Application::from_user_key(user_key);
    let call = Info::new(Type::Authorize, &service, &app, None);

    let result = call.to_request();
    let (res_endpoint, res_params) = parse_path(result.path());

    let mut expected_params = HashMap::new();
    expected_params.insert("provider_key", provider_key);
    expected_params.insert("service_id", service_id);
    expected_params.insert("user_key", user_key);

    assert_eq!("GET", result.method());
    assert_eq!("/transactions/authorize.xml", res_endpoint);
    assert_eq!(expected_params, res_params);
}

#[test]
fn returns_auth_request_from_service_id_pkey_user_key_and_user_id() {
    let provider_key = "a_provider_key";
    let creds = Credentials::from_key(provider_key);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let user_key = "a_user_key";
    let app = Application::from_user_key(user_key);
    let user_id = "a_user_id";
    let user = User::from_user_id(user_id);
    let call = Info::new(Type::Authorize, &service, &app, Some(&user));

    let result = call.to_request();
    let (res_endpoint, res_params) = parse_path(result.path());

    let mut expected_params = HashMap::new();
    expected_params.insert("provider_key", provider_key);
    expected_params.insert("service_id", service_id);
    expected_params.insert("user_key", user_key);
    expected_params.insert("user_id", user_id);

    assert_eq!("GET", result.method());
    assert_eq!("/transactions/authorize.xml", res_endpoint);
    assert_eq!(expected_params, res_params);
}

#[test]
fn returns_auth_request_from_service_id_pkey_user_key_and_oauth_user() {
    let provider_key = "a_provider_key";
    let creds = Credentials::from_key(provider_key);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let user_key = "a_user_key";
    let app = Application::from_user_key(user_key);
    let user_oauth_token = "a_user_token";
    let user = User::from_oauth_token(user_oauth_token);
    let call = Info::new(Type::Authorize, &service, &app, Some(&user));

    let result = call.to_request();
    let (res_endpoint, res_params) = parse_path(result.path());

    let mut expected_params = HashMap::new();
    expected_params.insert("provider_key", provider_key);
    expected_params.insert("service_id", service_id);
    expected_params.insert("user_key", user_key);
    expected_params.insert("access_token", user_oauth_token);

    assert_eq!("GET", result.method());
    assert_eq!("/transactions/oauth_authorize.xml", res_endpoint);
    assert_eq!(expected_params, res_params);
}

#[test]
fn returns_auth_request_from_service_id_pkey_and_oauth_token() {
    let provider_key = "a_provider_key";
    let creds = Credentials::from_key(provider_key);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let oauth_token = "an_app_token";
    let app = Application::from_oauth_token(oauth_token);
    let call = Info::new(Type::Authorize, &service, &app, None);

    let result = call.to_request();
    let (res_endpoint, res_params) = parse_path(result.path());

    let mut expected_params = HashMap::new();
    expected_params.insert("provider_key", provider_key);
    expected_params.insert("service_id", service_id);
    expected_params.insert("access_token", oauth_token);

    assert_eq!("GET", result.method());
    assert_eq!("/transactions/oauth_authorize.xml", res_endpoint);
    assert_eq!(expected_params, res_params);
}

#[test]
fn returns_auth_request_from_service_id_pkey_oauth_token_and_user_id() {
    let provider_key = "a_provider_key";
    let creds = Credentials::from_key(provider_key);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let oauth_token = "an_app_token";
    let app = Application::from_oauth_token(oauth_token);
    let user_id = "a_user_id";
    let user = User::from_user_id(user_id);
    let call = Info::new(Type::Authorize, &service, &app, Some(&user));

    let result = call.to_request();
    let (res_endpoint, res_params) = parse_path(result.path());

    let mut expected_params = HashMap::new();
    expected_params.insert("provider_key", provider_key);
    expected_params.insert("service_id", service_id);
    expected_params.insert("access_token", oauth_token);
    expected_params.insert("user_id", user_id);

    assert_eq!("GET", result.method());
    assert_eq!("/transactions/oauth_authorize.xml", res_endpoint);
    assert_eq!(expected_params, res_params);
}

#[test]
#[ignore]
fn returns_auth_request_from_service_id_pkey_oauth_token_and_oauth_user() {
    // TODO fix code. It should not be possible to create an Info instance with
    // using a token for the app and another for the user.
}

#[test]
fn returns_auth_request_from_service_id_token_and_app_id() {
    let service_token = "a_service_token";
    let creds = Credentials::from_token(service_token);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let app_id = "an_app_id";
    let app = Application::from_app_id(app_id);
    let call = Info::new(Type::Authorize, &service, &app, None);

    let result = call.to_request();
    let (res_endpoint, res_params) = parse_path(result.path());

    let mut expected_params = HashMap::new();
    expected_params.insert("service_token", service_token);
    expected_params.insert("service_id", service_id);
    expected_params.insert("app_id", app_id);

    assert_eq!("GET", result.method());
    assert_eq!("/transactions/authorize.xml", res_endpoint);
    assert_eq!(expected_params, res_params);
}

#[test]
fn returns_auth_request_from_service_id_token_app_id_and_user_id() {
    let service_token = "a_service_token";
    let creds = Credentials::from_token(service_token);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let app_id = "an_app_id";
    let app = Application::from_app_id(app_id);
    let user_id = "a_user_id";
    let user = User::from_user_id(user_id);
    let call = Info::new(Type::Authorize, &service, &app, Some(&user));

    let result = call.to_request();
    let (res_endpoint, res_params) = parse_path(result.path());

    let mut expected_params = HashMap::new();
    expected_params.insert("service_token", service_token);
    expected_params.insert("service_id", service_id);
    expected_params.insert("app_id", app_id);
    expected_params.insert("user_id", user_id);

    assert_eq!("GET", result.method());
    assert_eq!("/transactions/authorize.xml", res_endpoint);
    assert_eq!(expected_params, res_params);
}

#[test]
fn returns_auth_request_from_service_id_token_app_id_and_oauth_user() {
    let service_token = "a_service_token";
    let creds = Credentials::from_token(service_token);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let app_id = "an_app_id";
    let app = Application::from_app_id(app_id);
    let user_oauth_token = "a_user_token";
    let user = User::from_oauth_token(user_oauth_token);
    let call = Info::new(Type::Authorize, &service, &app, Some(&user));

    let result = call.to_request();
    let (res_endpoint, res_params) = parse_path(result.path());

    let mut expected_params = HashMap::new();
    expected_params.insert("service_token", service_token);
    expected_params.insert("service_id", service_id);
    expected_params.insert("app_id", app_id);
    expected_params.insert("access_token", user_oauth_token);

    assert_eq!("GET", result.method());
    assert_eq!("/transactions/oauth_authorize.xml", res_endpoint);
    assert_eq!(expected_params, res_params);
}

#[test]
fn returns_auth_request_from_service_id_token_app_id_and_app_key() {
    let service_token = "a_service_token";
    let creds = Credentials::from_token(service_token);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let app_id = "an_app_id";
    let app_key = "an_app_key";
    let app = Application::from_app_id_and_key(app_id, app_key);
    let call = Info::new(Type::Authorize, &service, &app, None);

    let result = call.to_request();
    let (res_endpoint, res_params) = parse_path(result.path());

    let mut expected_params = HashMap::new();
    expected_params.insert("service_token", service_token);
    expected_params.insert("service_id", service_id);
    expected_params.insert("app_id", app_id);
    expected_params.insert("app_key", app_key);

    assert_eq!("GET", result.method());
    assert_eq!("/transactions/authorize.xml", res_endpoint);
    assert_eq!(expected_params, res_params);
}

#[test]
fn returns_auth_request_from_service_id_token_app_id_app_key_and_user_id() {
    let service_token = "a_service_token";
    let creds = Credentials::from_token(service_token);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let app_id = "an_app_id";
    let app_key = "an_app_key";
    let app = Application::from_app_id_and_key(app_id, app_key);
    let user_id = "a_user_id";
    let user = User::from_user_id(user_id);
    let call = Info::new(Type::Authorize, &service, &app, Some(&user));

    let result = call.to_request();
    let (res_endpoint, res_params) = parse_path(result.path());

    let mut expected_params = HashMap::new();
    expected_params.insert("service_token", service_token);
    expected_params.insert("service_id", service_id);
    expected_params.insert("app_id", app_id);
    expected_params.insert("app_key", app_key);
    expected_params.insert("user_id", user_id);

    assert_eq!("GET", result.method());
    assert_eq!("/transactions/authorize.xml", res_endpoint);
    assert_eq!(expected_params, res_params);
}

#[test]
fn returns_auth_request_from_service_id_token_app_id_app_key_and_oauth_user() {
    let service_token = "a_service_token";
    let creds = Credentials::from_token(service_token);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let app_id = "an_app_id";
    let app_key = "an_app_key";
    let app = Application::from_app_id_and_key(app_id, app_key);
    let user_oauth_token = "a_user_token";
    let user = User::from_oauth_token(user_oauth_token);
    let call = Info::new(Type::Authorize, &service, &app, Some(&user));

    let result = call.to_request();
    let (res_endpoint, res_params) = parse_path(result.path());

    let mut expected_params = HashMap::new();
    expected_params.insert("service_token", service_token);
    expected_params.insert("service_id", service_id);
    expected_params.insert("app_id", app_id);
    expected_params.insert("app_key", app_key);
    expected_params.insert("access_token", user_oauth_token);

    assert_eq!("GET", result.method());
    assert_eq!("/transactions/oauth_authorize.xml", res_endpoint);
    assert_eq!(expected_params, res_params);
}

#[test]
fn returns_auth_request_from_service_id_token_and_user_key() {
    let service_token = "a_service_token";
    let creds = Credentials::from_token(service_token);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let user_key = "a_user_key";
    let app = Application::from_user_key(user_key);
    let call = Info::new(Type::Authorize, &service, &app, None);

    let result = call.to_request();
    let (res_endpoint, res_params) = parse_path(result.path());

    let mut expected_params = HashMap::new();
    expected_params.insert("service_token", service_token);
    expected_params.insert("service_id", service_id);
    expected_params.insert("user_key", user_key);

    assert_eq!("GET", result.method());
    assert_eq!("/transactions/authorize.xml", res_endpoint);
    assert_eq!(expected_params, res_params);
}

#[test]
fn returns_auth_request_from_service_id_token_user_key_and_user_id() {
    let service_token = "a_service_token";
    let creds = Credentials::from_token(service_token);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let user_key = "a_user_key";
    let app = Application::from_user_key(user_key);
    let user_id = "a_user_id";
    let user = User::from_user_id(user_id);
    let call = Info::new(Type::Authorize, &service, &app, Some(&user));

    let result = call.to_request();
    let (res_endpoint, res_params) = parse_path(result.path());

    let mut expected_params = HashMap::new();
    expected_params.insert("service_token", service_token);
    expected_params.insert("service_id", service_id);
    expected_params.insert("user_key", user_key);
    expected_params.insert("user_id", user_id);

    assert_eq!("GET", result.method());
    assert_eq!("/transactions/authorize.xml", res_endpoint);
    assert_eq!(expected_params, res_params);
}

#[test]
fn returns_auth_request_from_service_id_token_user_key_and_oauth_user() {
    let service_token = "a_service_token";
    let creds = Credentials::from_token(service_token);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let user_key = "a_user_key";
    let app = Application::from_user_key(user_key);
    let oauth_user_token = "a_user_token";
    let user = User::from_oauth_token(oauth_user_token);
    let call = Info::new(Type::Authorize, &service, &app, Some(&user));

    let result = call.to_request();
    let (res_endpoint, res_params) = parse_path(result.path());

    let mut expected_params = HashMap::new();
    expected_params.insert("service_token", service_token);
    expected_params.insert("service_id", service_id);
    expected_params.insert("user_key", user_key);
    expected_params.insert("access_token", oauth_user_token);

    assert_eq!("GET", result.method());
    assert_eq!("/transactions/oauth_authorize.xml", res_endpoint);
    assert_eq!(expected_params, res_params);
}

#[test]
fn returns_auth_request_from_service_id_token_and_oauth_token() {
    let service_token = "a_service_token";
    let creds = Credentials::from_token(service_token);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let oauth_token = "an_app_token";
    let app = Application::from_oauth_token(oauth_token);
    let call = Info::new(Type::Authorize, &service, &app, None);

    let result = call.to_request();
    let (res_endpoint, res_params) = parse_path(result.path());

    let mut expected_params = HashMap::new();
    expected_params.insert("service_token", service_token);
    expected_params.insert("service_id", service_id);
    expected_params.insert("access_token", oauth_token);

    assert_eq!("GET", result.method());
    assert_eq!("/transactions/oauth_authorize.xml", res_endpoint);
    assert_eq!(expected_params, res_params);
}

#[test]
fn returns_auth_request_from_service_id_token_oauth_token_and_user_id() {
    let service_token = "a_service_token";
    let creds = Credentials::from_token(service_token);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let oauth_token = "an_app_token";
    let app = Application::from_oauth_token(oauth_token);
    let user_id = "a_user_id";
    let user = User::from_user_id(user_id);
    let call = Info::new(Type::Authorize, &service, &app, Some(&user));

    let result = call.to_request();
    let (res_endpoint, res_params) = parse_path(result.path());

    let mut expected_params = HashMap::new();
    expected_params.insert("service_token", service_token);
    expected_params.insert("service_id", service_id);
    expected_params.insert("access_token", oauth_token);
    expected_params.insert("user_id", user_id);

    assert_eq!("GET", result.method());
    assert_eq!("/transactions/oauth_authorize.xml", res_endpoint);
    assert_eq!(expected_params, res_params);
}

#[test]
#[ignore]
fn returns_auth_request_from_service_id_token_oauth_token_and_oauth_user() {
    // TODO fix code. It should not be possible to create an Info instance with
    // using a token for the app and another for the user.
}

/// Given the path of a Request instance, returns a tuple where the first
/// element is the endpoint included in the path, and the second is a HashMap
/// with the parameters and their values.
fn parse_path(path: &str) -> (String, HashMap<&str, &str>) {
    let split_path: Vec<&str> = path.split("?").collect();
    let endpoint = split_path[0];
    let url_params = split_path[1];

    let mut res_params = HashMap::new();

    let param_values: Vec<Vec<&str>> = url_params
        .split("&")
        .map(|param| param.split("=").collect()).collect();

    for param_val in param_values {
        res_params.insert(param_val[0], param_val[1]);
    }

    (endpoint.to_owned(), res_params)
}
