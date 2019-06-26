use threescalers::api_call::*;
use threescalers::application::*;
use threescalers::credentials::*;
use threescalers::service::*;
use threescalers::user::*;

use std::collections::HashMap;

use crate::helpers::*;
use threescalers::transaction::Transaction;

#[test]
fn returns_auth_request_from_service_id_pkey_and_app_id() {
    let provider_key = "a_provider_key";
    let creds = Credentials::from_key(provider_key);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let app_id = "an_app_id";
    let app = Application::from_app_id(app_id);
    let txn = [Transaction::new(&app, None, None, None)];
    let call = ApiCall::new(Kind::Authorize, &service, &txn, None);

    let mut expected_params = HashMap::new();
    expected_params.insert("provider_key", provider_key);
    expected_params.insert("service_id", service_id);
    expected_params.insert("app_id", app_id);

    //let request = call.to_request();

    //assert_eq!("GET", request.method());
    //assert_eq!("/transactions/authorize.xml", request.endpoint());
    assert_eq!(expected_params, vec_to_hash(&call.params()));
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
    let txn = [Transaction::new(&app, Some(&user), None, None)];
    let call = ApiCall::new(Kind::Authorize, &service, &txn, None);

    let mut expected_params = HashMap::new();
    expected_params.insert("provider_key", provider_key);
    expected_params.insert("service_id", service_id);
    expected_params.insert("app_id", app_id);
    expected_params.insert("user_id", user_id);

    //let request = call.to_request();

    //assert_eq!("GET", request.method());
    //assert_eq!("/transactions/authorize.xml", request.endpoint());
    assert_eq!(expected_params, vec_to_hash(&call.params()));
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
    let txn = [Transaction::new(&app, Some(&user), None, None)];
    let call = ApiCall::new(Kind::Authorize, &service, &txn, None);

    let mut expected_params = HashMap::new();
    expected_params.insert("provider_key", provider_key);
    expected_params.insert("service_id", service_id);
    expected_params.insert("app_id", app_id);
    expected_params.insert("access_token", user_oauth_token);

    //let request = call.to_request();

    //assert_eq!("GET", request.method());
    //assert_eq!("/transactions/oauth_authorize.xml", request.endpoint());
    assert_eq!(expected_params, vec_to_hash(&call.params()));
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
    let txn = [Transaction::new(&app, None, None, None)];
    let call = ApiCall::new(Kind::Authorize, &service, &txn, None);

    let mut expected_params = HashMap::new();
    expected_params.insert("provider_key", provider_key);
    expected_params.insert("service_id", service_id);
    expected_params.insert("app_id", app_id);
    expected_params.insert("app_key", app_key);

    //let request = call.to_request();

    //assert_eq!("GET", request.method());
    //assert_eq!("/transactions/authorize.xml", request.endpoint());
    assert_eq!(expected_params, vec_to_hash(&call.params()));
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
    let txn = [Transaction::new(&app, Some(&user), None, None)];
    let call = ApiCall::new(Kind::Authorize, &service, &txn, None);

    let mut expected_params = HashMap::new();
    expected_params.insert("provider_key", provider_key);
    expected_params.insert("service_id", service_id);
    expected_params.insert("app_id", app_id);
    expected_params.insert("app_key", app_key);
    expected_params.insert("user_id", user_id);

    //let request = call.to_request();

    //assert_eq!("GET", request.method());
    //assert_eq!("/transactions/authorize.xml", request.endpoint());
    assert_eq!(expected_params, vec_to_hash(&call.params()));
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
    let txn = [Transaction::new(&app, Some(&user), None, None)];
    let call = ApiCall::new(Kind::Authorize, &service, &txn, None);

    let mut expected_params = HashMap::new();
    expected_params.insert("provider_key", provider_key);
    expected_params.insert("service_id", service_id);
    expected_params.insert("app_id", app_id);
    expected_params.insert("app_key", app_key);
    expected_params.insert("access_token", user_oauth_token);

    //let request = call.to_request();

    //assert_eq!("GET", request.method());
    //assert_eq!("/transactions/oauth_authorize.xml", request.endpoint());
    assert_eq!(expected_params, vec_to_hash(&call.params()));
}

#[test]
fn returns_auth_request_from_service_id_pkey_and_user_key() {
    let provider_key = "a_provider_key";
    let creds = Credentials::from_key(provider_key);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let user_key = "a_user_key";
    let app = Application::from_user_key(user_key);
    let txn = [Transaction::new(&app, None, None, None)];
    let call = ApiCall::new(Kind::Authorize, &service, &txn, None);

    let mut expected_params = HashMap::new();
    expected_params.insert("provider_key", provider_key);
    expected_params.insert("service_id", service_id);
    expected_params.insert("user_key", user_key);

    //let request = call.to_request();

    //assert_eq!("GET", request.method());
    //assert_eq!("/transactions/authorize.xml", request.endpoint());
    assert_eq!(expected_params, vec_to_hash(&call.params()));
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
    let txn = [Transaction::new(&app, Some(&user), None, None)];
    let call = ApiCall::new(Kind::Authorize, &service, &txn, None);

    let mut expected_params = HashMap::new();
    expected_params.insert("provider_key", provider_key);
    expected_params.insert("service_id", service_id);
    expected_params.insert("user_key", user_key);
    expected_params.insert("user_id", user_id);

    //let request = call.to_request();

    //assert_eq!("GET", request.method());
    //assert_eq!("/transactions/authorize.xml", request.endpoint());
    assert_eq!(expected_params, vec_to_hash(&call.params()));
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
    let txn = [Transaction::new(&app, Some(&user), None, None)];
    let call = ApiCall::new(Kind::Authorize, &service, &txn, None);

    let mut expected_params = HashMap::new();
    expected_params.insert("provider_key", provider_key);
    expected_params.insert("service_id", service_id);
    expected_params.insert("user_key", user_key);
    expected_params.insert("access_token", user_oauth_token);

    //let request = call.to_request();

    //assert_eq!("GET", request.method());
    //assert_eq!("/transactions/oauth_authorize.xml", request.endpoint());
    assert_eq!(expected_params, vec_to_hash(&call.params()));
}

#[test]
fn returns_auth_request_from_service_id_pkey_and_oauth_token() {
    let provider_key = "a_provider_key";
    let creds = Credentials::from_key(provider_key);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let oauth_token = "an_app_token";
    let app = Application::from_oauth_token(oauth_token);
    let txn = [Transaction::new(&app, None, None, None)];
    let call = ApiCall::new(Kind::Authorize, &service, &txn, None);

    let mut expected_params = HashMap::new();
    expected_params.insert("provider_key", provider_key);
    expected_params.insert("service_id", service_id);
    expected_params.insert("access_token", oauth_token);

    //let request = call.to_request();

    //assert_eq!("GET", request.method());
    //assert_eq!("/transactions/oauth_authorize.xml", request.endpoint());
    assert_eq!(expected_params, vec_to_hash(&call.params()));
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
    let txn = [Transaction::new(&app, Some(&user), None, None)];
    let call = ApiCall::new(Kind::Authorize, &service, &txn, None);

    let mut expected_params = HashMap::new();
    expected_params.insert("provider_key", provider_key);
    expected_params.insert("service_id", service_id);
    expected_params.insert("access_token", oauth_token);
    expected_params.insert("user_id", user_id);

    //let request = call.to_request();

    //assert_eq!("GET", request.method());
    //assert_eq!("/transactions/oauth_authorize.xml", request.endpoint());
    assert_eq!(expected_params, vec_to_hash(&call.params()));
}

#[test]
#[ignore]
fn returns_auth_request_from_service_id_pkey_oauth_token_and_oauth_user() {
    // TODO fix code. It should not be possible to create an ApiCall instance with
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
    let txn = [Transaction::new(&app, None, None, None)];
    let call = ApiCall::new(Kind::Authorize, &service, &txn, None);

    let mut expected_params = HashMap::new();
    expected_params.insert("service_token", service_token);
    expected_params.insert("service_id", service_id);
    expected_params.insert("app_id", app_id);

    //let request = call.to_request();

    //assert_eq!("GET", request.method());
    //assert_eq!("/transactions/authorize.xml", request.endpoint());
    assert_eq!(expected_params, vec_to_hash(&call.params()));
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
    let txn = [Transaction::new(&app, Some(&user), None, None)];
    let call = ApiCall::new(Kind::Authorize, &service, &txn, None);

    let mut expected_params = HashMap::new();
    expected_params.insert("service_token", service_token);
    expected_params.insert("service_id", service_id);
    expected_params.insert("app_id", app_id);
    expected_params.insert("user_id", user_id);

    //let request = call.to_request();

    //assert_eq!("GET", request.method());
    //assert_eq!("/transactions/authorize.xml", request.endpoint());
    assert_eq!(expected_params, vec_to_hash(&call.params()));
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
    let txn = [Transaction::new(&app, Some(&user), None, None)];
    let call = ApiCall::new(Kind::Authorize, &service, &txn, None);

    let mut expected_params = HashMap::new();
    expected_params.insert("service_token", service_token);
    expected_params.insert("service_id", service_id);
    expected_params.insert("app_id", app_id);
    expected_params.insert("access_token", user_oauth_token);

    //let request = call.to_request();

    //assert_eq!("GET", request.method());
    //assert_eq!("/transactions/oauth_authorize.xml", request.endpoint());
    assert_eq!(expected_params, vec_to_hash(&call.params()));
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
    let txn = [Transaction::new(&app, None, None, None)];
    let call = ApiCall::new(Kind::Authorize, &service, &txn, None);

    let mut expected_params = HashMap::new();
    expected_params.insert("service_token", service_token);
    expected_params.insert("service_id", service_id);
    expected_params.insert("app_id", app_id);
    expected_params.insert("app_key", app_key);

    //let request = call.to_request();

    //assert_eq!("GET", request.method());
    //assert_eq!("/transactions/authorize.xml", request.endpoint());
    assert_eq!(expected_params, vec_to_hash(&call.params()));
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
    let txn = [Transaction::new(&app, Some(&user), None, None)];
    let call = ApiCall::new(Kind::Authorize, &service, &txn, None);

    let mut expected_params = HashMap::new();
    expected_params.insert("service_token", service_token);
    expected_params.insert("service_id", service_id);
    expected_params.insert("app_id", app_id);
    expected_params.insert("app_key", app_key);
    expected_params.insert("user_id", user_id);

    //let request = call.to_request();

    //assert_eq!("GET", request.method());
    //assert_eq!("/transactions/authorize.xml", request.endpoint());
    assert_eq!(expected_params, vec_to_hash(&call.params()));
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
    let txn = [Transaction::new(&app, Some(&user), None, None)];
    let call = ApiCall::new(Kind::Authorize, &service, &txn, None);

    let mut expected_params = HashMap::new();
    expected_params.insert("service_token", service_token);
    expected_params.insert("service_id", service_id);
    expected_params.insert("app_id", app_id);
    expected_params.insert("app_key", app_key);
    expected_params.insert("access_token", user_oauth_token);

    //let request = call.to_request();

    //assert_eq!("GET", request.method());
    //assert_eq!("/transactions/oauth_authorize.xml", request.endpoint());
    assert_eq!(expected_params, vec_to_hash(&call.params()));
}

#[test]
fn returns_auth_request_from_service_id_token_and_user_key() {
    let service_token = "a_service_token";
    let creds = Credentials::from_token(service_token);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let user_key = "a_user_key";
    let app = Application::from_user_key(user_key);
    let txn = [Transaction::new(&app, None, None, None)];
    let call = ApiCall::new(Kind::Authorize, &service, &txn, None);

    let mut expected_params = HashMap::new();
    expected_params.insert("service_token", service_token);
    expected_params.insert("service_id", service_id);
    expected_params.insert("user_key", user_key);

    //let request = call.to_request();

    //assert_eq!("GET", request.method());
    //assert_eq!("/transactions/authorize.xml", request.endpoint());
    assert_eq!(expected_params, vec_to_hash(&call.params()));
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
    let txn = [Transaction::new(&app, Some(&user), None, None)];
    let call = ApiCall::new(Kind::Authorize, &service, &txn, None);

    let mut expected_params = HashMap::new();
    expected_params.insert("service_token", service_token);
    expected_params.insert("service_id", service_id);
    expected_params.insert("user_key", user_key);
    expected_params.insert("user_id", user_id);

    //let request = call.to_request();

    //assert_eq!("GET", request.method());
    //assert_eq!("/transactions/authorize.xml", request.endpoint());
    assert_eq!(expected_params, vec_to_hash(&call.params()));
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
    let txn = [Transaction::new(&app, Some(&user), None, None)];
    let call = ApiCall::new(Kind::Authorize, &service, &txn, None);

    let mut expected_params = HashMap::new();
    expected_params.insert("service_token", service_token);
    expected_params.insert("service_id", service_id);
    expected_params.insert("user_key", user_key);
    expected_params.insert("access_token", oauth_user_token);

    //let request = call.to_request();

    //assert_eq!("GET", request.method());
    //assert_eq!("/transactions/oauth_authorize.xml", request.endpoint());
    assert_eq!(expected_params, vec_to_hash(&call.params()));
}

#[test]
fn returns_auth_request_from_service_id_token_and_oauth_token() {
    let service_token = "a_service_token";
    let creds = Credentials::from_token(service_token);
    let service_id = "a_service_id";
    let service = Service::new(service_id, creds);
    let oauth_token = "an_app_token";
    let app = Application::from_oauth_token(oauth_token);
    let txn = [Transaction::new(&app, None, None, None)];
    let call = ApiCall::new(Kind::Authorize, &service, &txn, None);

    let mut expected_params = HashMap::new();
    expected_params.insert("service_token", service_token);
    expected_params.insert("service_id", service_id);
    expected_params.insert("access_token", oauth_token);

    //let request = call.to_request();

    //assert_eq!("GET", request.method());
    //assert_eq!("/transactions/oauth_authorize.xml", request.endpoint());
    assert_eq!(expected_params, vec_to_hash(&call.params()));
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
    let txn = [Transaction::new(&app, Some(&user), None, None)];
    let call = ApiCall::new(Kind::Authorize, &service, &txn, None);

    let mut expected_params = HashMap::new();
    expected_params.insert("service_token", service_token);
    expected_params.insert("service_id", service_id);
    expected_params.insert("access_token", oauth_token);
    expected_params.insert("user_id", user_id);

    //let request = call.to_request();

    //assert_eq!("GET", request.method());
    //assert_eq!("/transactions/oauth_authorize.xml", request.endpoint());
    assert_eq!(expected_params, vec_to_hash(&call.params()));
}

#[test]
#[ignore]
fn returns_auth_request_from_service_id_token_oauth_token_and_oauth_user() {
    // TODO fix code. It should not be possible to create an ApiCall instance with
    // using a token for the app and another for the user.
}

mod helpers {
    use std::borrow::Cow;
    use std::collections::HashMap;
    use std::hash::Hash;

    pub fn vec_to_hash<'a, V: Copy>(vec: &'a Vec<(Cow<str>, V)>) -> HashMap<&'a str, V> {
        let mut h: HashMap<&str, V> = HashMap::new();
        for (k, v) in vec.iter() {
            h.insert(k.as_ref(), *v);
        }
        h
    }
}
