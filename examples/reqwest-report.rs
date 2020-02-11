use threescalers::{
    api_call::*,
    application::*,
    credentials::*,
    extensions::Extensions,
    http::{
        request::SetupRequest,
        Request,
    },
    service::*,
    transaction::Transaction,
    usage::Usage,
};

use reqwest::blocking::{
    Client,
    RequestBuilder,
    Response,
};

fn main() -> Result<(), threescalers::errors::Error> {
    let creds = Credentials::ServiceToken(ServiceToken::from("12[3]token"));
    let svc = Service::new("svc123", creds);
    let uks = ["userkey_1", "userkey_2", "userkey_3", "userkey 4", "userkey 5"];
    let apps = uks.iter()
                  .map(|uk| Application::from(UserKey::from(*uk)))
                  .collect::<Vec<_>>();

    println!("Apps: {:#?}", apps);

    let usages = [("metric11", 11),
                  ("metric12", 12),
                  ("metric21", 21),
                  ("metric22", 22),
                  ("metric31", 31),
                  ("metric32", 32),
                  ("metric41", 41),
                  ("metric42", 42),
                  ("metric51", 51),
                  ("metric52", 52)].iter()
                                   .map(|m| (m.0, format!("{}", m.1)))
                                   .collect::<Vec<_>>();

    let usages = usages.chunks(2)
                       .map(|metrics_and_values| Usage::from(metrics_and_values))
                       .collect::<Vec<_>>();

    println!("Usages: {:#?}", usages);

    let ts = Default::default();

    let txns = apps.iter()
                   .zip(&usages)
                   .map(|(a, u)| Transaction::new(a, None, Some(u), Some(&ts)))
                   .collect::<Vec<_>>();

    let mut extensions = Extensions::new();
    extensions.insert("no_body", "1");
    extensions.insert("testing[=]", "0[=:=]0");
    let mut apicall = ApiCall::builder(&svc);
    let apicall = apicall.transactions(&txns)
                         .extensions(&extensions)
                         .kind(Kind::Report)
                         .build()?;
    let request = Request::from(&apicall);

    println!("apicall: {:#?}", apicall);
    println!("request: {:#?}", request);

    let _ = run_request(request);

    Ok(())
}

fn run_request(request: Request) -> Result<Response, reqwest::Error> {
    let mut client = Client::new();
    let reqbuilder = client.setup_request(request, "https://echo-api.3scale.net");
    let result = exec_request(reqbuilder);
    show_response(result)
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
            let jsonval = serde_json::from_reader::<&mut Response, serde_json::Value>(&mut response).unwrap();
            println!("*** BODY ***\n{}",
                     serde_json::to_string_pretty(&jsonval).unwrap());
            Ok(response)
        }
        Err(e) => {
            println!("*** ERROR ***\n{:#?}", e);
            Err(e)
        }
    }
}
