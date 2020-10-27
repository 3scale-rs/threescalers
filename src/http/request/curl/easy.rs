use std::prelude::v1::*;

use crate::Error;

use super::super::{
    Method,
    Request,
    SetupRequest,
};
use curl::easy::{
    Easy,
    List,
    Transfer,
};

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
        if let Self::Easy(e) = self { Some(e) } else { None }
    }
}

impl<'easy, 'data, URI: ToString> SetupRequest<'easy, URI, Result<CurlEasyClient<'easy, 'data>, Error>>
    for Easy
{
    fn setup_request(&'easy mut self,
                     r: Request,
                     params: URI)
                     -> Result<CurlEasyClient<'easy, 'data>, Error> {
        use core::convert::TryFrom;

        let (uri, body) = r.parameters.uri_and_body(r.path);
        let uri_base = params;
        let uri = uri_base.to_string() + uri.as_ref();

        match r.method {
            Method::GET => self.get(true),
            Method::POST => self.post(true),
            Method::PUT => self.put(true),
            // any other verb needs to use custom_request()
            m => self.custom_request(m.as_str()),
        }?;

        self.url(uri.as_str())?;
        let mut headerlist = List::try_from(&r.headers)?;
        // libcurl by default adds "Expect: 100-continue" to send bodies, which would break us
        headerlist.append("Expect:")?;
        // don't specify Content-Type for this request (similar to other clients)
        headerlist.append("Content-Type:")?;
        self.http_headers(headerlist)?;

        Ok(match body {
            Some(_) => {
                let body = r.parameters.into_inner();
                // this sets the Content-Length - some servers will misbehave without this
                self.post_field_size(body.len() as u64)?;
                let mut transfer = self.transfer();

                let mut count = 0usize;
                transfer.read_function(move |buf| Ok(super::copy_data(&mut count, &body.as_bytes(), buf)))?;

                transfer.into()
            }
            None => (self as &Easy).into(),
        })
    }
}
