use std::prelude::v1::*;

use crate::{anyhow, Error, Result};

use super::super::{Method, Request, SetupRequest};
use curl::easy::{Easy2, Handler, List, ReadError};

/// This trait has to be implemented by the Easy2<H>'s H generic type, as well as curl's Handler.
/// This is because the body of POST requests needs to be pushed from the storage associated to
/// that type, so this trait provides a method to store the body String and a method to copy data
/// by providing just an offset as multiple calls to push the data happen.
pub trait SetBody: Handler {
    /// Static method to copy data over starting from an offset, return the number of bytes that
    /// were copied and updating the offset.
    #[inline]
    fn copy_data(offset: &mut usize, source: &[u8], dest: &mut [u8]) -> usize {
        super::copy_data(offset, source, dest)
    }

    /// This method should store the body data to be pushed by this request.
    fn set_body(&mut self, body: String);
}

/// A default type that works with the requirements of conversion between a Request and a set up
/// Easy2 client by implementing the `SetBody` trait.
#[derive(Debug, Clone)]
pub struct BodyHandle {
    count: usize,
    body: Option<String>,
}

impl BodyHandle {
    pub fn new() -> Self {
        Self {
            count: 0,
            body: None,
        }
    }

    /// A method to set/unset the body. To be used from a `SetBody` trait impl.
    pub fn with_body(&mut self, body: Option<String>) {
        self.body = body;
    }
}

impl Default for BodyHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl Handler for BodyHandle {
    fn read(&mut self, data: &mut [u8]) -> Result<usize, ReadError> {
        // we should never have this called on requests without a body
        debug_assert!(self.body.is_some());

        if let Some(ref body) = self.body {
            Ok(Self::copy_data(&mut self.count, body.as_bytes(), data))
        } else {
            unreachable!()
        }
    }
}

impl SetBody for BodyHandle {
    fn set_body(&mut self, body: String) {
        self.with_body(body.into());
    }
}

impl<URI: ToString, H: SetBody> SetupRequest<'_, URI, Result<(), Error>> for Easy2<H> {
    fn setup_request(&mut self, r: Request, params: URI) -> Result<(), Error> {
        let (uri, body) = r.parameters.uri_and_body(r.path);
        let uri_base = params;
        let uri = uri_base.to_string() + uri.as_ref();

        match r.method {
            Method::GET => self.get(true),
            Method::POST => self.post(true),
            Method::PUT => self.put(true),
            // any other verb needs to use custom_request()
            m => self.custom_request(m.as_str()),
        }
        .map_err(|e| anyhow!("failed to set curl request method: {:#?}", e))?;

        self.url(uri.as_str())
            .map_err(|e| anyhow!("failed to set curl request URL: {:#?}", e))?;
        let mut headerlist = List::try_from(&r.headers)
            .map_err(|e| anyhow!("failed to create curl::List from headers: {:#?}", e))?;
        // libcurl by default adds "Expect: 100-continue" to send bodies, which would break us
        headerlist
            .append("Expect:")
            .map_err(|e| anyhow!("failed to add node to curl::List: {:#?}", e))?;
        // don't specify Content-Type for this request (similar to other clients)
        headerlist
            .append("Content-Type:")
            .map_err(|e| anyhow!("failed to add node to curl::List: {:#?}", e))?;
        self.http_headers(headerlist)
            .map_err(|e| anyhow!("failed to add headers to curl client: {:#?}", e))?;

        if body.is_some() {
            let body = r.parameters.into_inner();
            // this sets the Content-Length - some servers will misbehave without this
            self.post_field_size(body.len() as u64)
                .map_err(|e| anyhow!("failed to set Content-Length: {:#?}", e))?;

            self.get_mut().set_body(body);
        }

        Ok(())
    }
}
