#[cfg(feature = "curl-easy")]
pub use easy::CurlEasyClient;

#[cfg(feature = "curl-easy")]
mod easy {
    use super::super::{FromRequest, Request};
    use curl::easy::{Easy, List, Transfer};
    use http_types::header::HeaderValue;
    use http_types::Method;

    fn headermap_to_curl_list(headermap: &http_types::HeaderMap<HeaderValue>) -> List {
        let mut list = List::new();
        headermap.iter().for_each(|(k, v)| {
            // this will scan for printable US-ASCII only bytes
            let header = v
                .to_str()
                .map(|hval| [k.as_str(), ": ", hval].concat())
                .expect("found header value without a displayable US-ASCII string");
            list.append(header.as_str())
                .expect("failed to allocate node for curl list of headers");
        });
        list
    }

    #[derive(Debug)]
    pub enum CurlEasyClient<'easy, 'data> {
        Easy(&'easy Easy),
        Transfer(Transfer<'easy, 'data>),
    }

    impl<'easy, 'data> From<Transfer<'easy, 'data>> for CurlEasyClient<'easy, 'data> {
        fn from(t: Transfer<'easy, 'data>) -> Self {
            Self::Transfer(t)
        }
    }

    impl<'easy> From<&'easy Easy> for CurlEasyClient<'easy, '_> {
        fn from(e: &'easy Easy) -> Self {
            Self::Easy(e)
        }
    }

    impl<'easy, 'data> CurlEasyClient<'easy, 'data> {
        pub fn new(easy: &'easy Easy) -> Self {
            Self::Easy(easy)
        }

        pub fn perform(&self) -> Result<(), curl::Error> {
            match self {
                Self::Easy(e) => e.perform(),
                Self::Transfer(t) => t.perform(),
            }
        }

        pub fn into_transfer(self) -> Option<Transfer<'easy, 'data>> {
            match self {
                Self::Transfer(t) => Some(t),
                Self::Easy(_) => None,
            }
        }

        pub fn easy(&self) -> Option<&'easy Easy> {
            if let Self::Easy(e) = self {
                Some(e)
            } else {
                None
            }
        }
    }

    impl<'easy, 'data, URI: ToString> FromRequest<(&'easy mut Easy, URI)>
        for CurlEasyClient<'easy, 'data>
    {
        fn from_request(r: Request, params: (&'easy mut Easy, URI)) -> Self {
            let (uri, body) = r.parameters.uri_and_body(r.path);
            let (client, uri_base) = params;
            let uri = uri_base.to_string() + uri.as_ref();

            match r.method {
                Method::GET => client.get(true),
                Method::POST => client.post(true),
                Method::PUT => client.put(true),
                // any other verb needs to use custom_request()
                m => client.custom_request(m.as_str()),
            }
            .expect("failed to set up the request's method");

            client
                .url(uri.as_str())
                .expect("error setting up url for curl");
            let mut headerlist = headermap_to_curl_list(&r.headers);
            // libcurl by default adds "Expect: 100-continue" to send bodies, which would break us
            headerlist
                .append("Expect:")
                .expect("failed to allocate node for curl list of headers");
            // don't specify Content-Type for this request (similar to other clients)
            headerlist
                .append("Content-Type:")
                .expect("failed to allocate node for curl list of headers");
            client
                .http_headers(headerlist)
                .expect("error setting up headers for curl");

            match body {
                Some(_) => {
                    use std::io::Read;

                    let body = r.parameters.into_inner();
                    // this sets the Content-Length - some servers will misbehave without this
                    client
                        .post_field_size(body.len() as u64)
                        .expect("failed to set post size");
                    let mut transfer = client.transfer();

                    let mut count = 0usize;
                    transfer
                        .read_function(move |buf| {
                            let mut bytes = &body.as_bytes()[count..];
                            let newcount = bytes
                                .read(buf)
                                .expect("error while copying body data to buffer");
                            count += newcount;
                            Ok(newcount)
                        })
                        .expect("failed to set up read function for curl");

                    transfer.into()
                }
                None => (client as &Easy).into(),
            }
        }
    }
}
