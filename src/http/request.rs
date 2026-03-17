extern crate reqwest;
extern crate serde_json;

use super::{HTTPHeader, HTTPPayload};
use reqwest::{
    blocking::{Client, RequestBuilder, Response},
    header::HeaderMap,
};
use serde_json::Value as Json;
use std::sync::OnceLock;

static CLIENT: OnceLock<Client> = OnceLock::new();

// TODO: How could I refacto the four function in a single one passing Client::get/post ?
pub struct HTTPRequest;

impl HTTPRequest {
    pub fn get_form(
        endpoint: &str,
        header: HTTPHeader,
        payload: HTTPPayload,
    ) -> Result<String, reqwest::Error> {
        // HTTPHeader must be converted to the reqwest HeaderMap struct..
        let header: HeaderMap = header.into();

        let client = Self::client();
        let request: RequestBuilder = client.get(endpoint).headers(header).form(&payload);

        let response: Response = request.send()?;
        response.text()
    }

    pub fn post_form(
        endpoint: &str,
        header: HTTPHeader,
        payload: HTTPPayload,
    ) -> Result<String, reqwest::Error> {
        // HTTPHeader must be converted to the reqwest HeaderMap struct..
        let header: HeaderMap = header.into();

        let client = Self::client();
        let request: RequestBuilder = client.post(endpoint).headers(header).form(&payload);

        let response: Response = request.send()?;
        response.text()
    }

    pub fn get_json(endpoint: &str, header: HTTPHeader) -> Result<Json, reqwest::Error> {
        // HTTPHeader must be converted to the reqwest HeaderMap struct..
        let header: HeaderMap = header.into();

        let client = Self::client();
        let request: RequestBuilder = client.get(endpoint).headers(header);

        let response: Response = request.send()?;
        response.json()
    }

    pub fn post_json(
        endpoint: &str,
        header: HTTPHeader,
        payload: HTTPPayload,
    ) -> Result<Json, reqwest::Error> {
        // HTTPHeader must be converted to the reqwest HeaderMap struct..
        let header: HeaderMap = header.into();

        let client = Self::client();

        let payload = payload.parse_json();

        let request: RequestBuilder = client.post(endpoint).headers(header).json(&payload);

        let response: Response = request.send()?;
        response.json()
    }

    fn client() -> &'static Client {
        CLIENT.get_or_init(Client::new)
    }
}
