use threescalers::{
    api_call::*,
    application::*,
    credentials::*,
    extensions::{self, Extension},
    http::{
        request::{curl::CurlEasyClient, SetupRequest},
        Request,
    },
    service::*,
    transaction::Transaction,
    usage::Usage,
};

use std::error::Error;

use curl::easy::Easy;

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

    let usages = usages.chunks(2).map(Usage::from).collect::<Vec<_>>();

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

fn run_request(request: Request) -> Result<(), Box<dyn Error>> {
    let mut client = Easy::new();
    let _ = client.verbose(true).unwrap();
    let curlclient = client.setup_request(request, "https://echo-api.3scale.net")?;
    let result = exec_request(&curlclient);
    show_response(curlclient, result).map_err(Into::into)
}

fn exec_request(curlc: &CurlEasyClient) -> Result<(), curl::Error> {
    println!("CurlClient: {:#?}", curlc);
    curlc.perform()
}

// Not looking directly at the response but using the verbose mode.
fn show_response(curlc: CurlEasyClient, res: Result<(), curl::Error>) -> Result<(), curl::Error> {
    match res {
        Ok(_) => {
            println!("*** SUCCESS ***\n{:#?}", curlc);
            Ok(())
        }
        Err(e) => {
            println!("*** ERROR ***\n{:#?}", e);
            Err(e)
        }
    }
}
