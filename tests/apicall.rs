extern crate threescalers;

use threescalers::request::*;
use threescalers::apicall::*;
use threescalers::service::*;
use threescalers::application::*;

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
