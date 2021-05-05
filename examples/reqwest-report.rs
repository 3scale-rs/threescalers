use threescalers::{
    api_call::*,
    application::*,
    credentials::*,
    extensions::{self, Extension},
    http::{request::SetupRequest, Request},
    service::*,
    transaction::Transaction,
    usage::Usage,
};

use std::error::Error;

use reqwest::blocking::{Client, RequestBuilder, Response};

fn main() -> Result<(), Box<dyn Error>> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let creds = Credentials::ServiceToken(ServiceToken::from("12[3]token"));
    let svc = Service::new("svc123", creds);
    let uks = [
        "userkey_1",
        "userkey_2",
        "userkey_3",
        "userkey 4",
        "userkey 5",
    ];
    let apps = uks
        .iter()
        .map(|uk| Application::from(UserKey::from(*uk)))
        .collect::<Vec<_>>();

    println!("Apps: {:#?}", apps);

    let usages = [
        ("metric11", 11),
        ("metric12", 12),
        ("metric21", 21),
        ("metric22", 22),
        ("metric31", 31),
        ("metric32", 32),
        ("metric41", 41),
        ("metric42", 42),
        ("metric51", 51),
        ("metric52", 52),
    ]
    .iter()
    .map(|m| (m.0, format!("{}", m.1)))
    .collect::<Vec<_>>();

    let usages = usages
        .chunks(2)
        .map(|metrics_and_values| Usage::from(metrics_and_values))
        .collect::<Vec<_>>();

    println!("Usages: {:#?}", usages);

    let ts = SystemTime::now().duration_since(UNIX_EPOCH).ok().map(|st| {
        std::convert::TryInto::<i64>::try_into(st.as_secs())
            .expect("cannot fit timestamp in an i64")
    });

    let txns = apps
        .iter()
        .zip(&usages)
        .map(|(a, u)| Transaction::new(a, None, Some(u), ts))
        .collect::<Vec<_>>();

    let extensions = extensions::List::new()
        .no_body()
        .push(Extension::Hierarchy)
        .push_other("testing[=]".into(), "0[=:=]0".into());
    let mut apicall = ApiCall::builder(&svc);
    let apicall = apicall
        .transactions(&txns)
        .extensions(&extensions)
        .kind(Kind::Report)
        .build()?;
    let request = Request::from(&apicall);

    println!("apicall: {:#?}", apicall);
    println!("request: {:#?}", request);

    let _ = run_request(request);

    Ok(())
}

fn run_request(request: Request) -> Result<Response, Box<dyn Error>> {
    let mut client = Client::new();
    let reqbuilder = client.setup_request(request, "https://echo-api.3scale.net")?;
    let result = exec_request(reqbuilder);
    show_response(result).map_err(Into::into)
}

fn exec_request(reqb: RequestBuilder) -> Result<Response, reqwest::Error> {
    println!("RequestBuilder: {:#?}", reqb);
    reqb.send()
}

fn show_response(res: Result<Response, reqwest::Error>) -> Result<Response, reqwest::Error> {
    match res {
        Ok(mut response) => {
            println!("*** SUCCESS ***\n{:#?}", response);
            // Response#json consumes the response in reqwest 0.10+, so use serde_json directly
            let jsonval =
                serde_json::from_reader::<&mut Response, serde_json::Value>(&mut response).unwrap();
            println!(
                "*** BODY ***\n{}",
                serde_json::to_string_pretty(&jsonval).unwrap()
            );
            Ok(response)
        }
        Err(e) => {
            println!("*** ERROR ***\n{:#?}", e);
            Err(e)
        }
    }
}
