extern crate reqwest;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::{convert::Into, str::FromStr};

pub struct HTTPHeader {
    header_map: HeaderMap,
}

impl HTTPHeader {
    pub fn new() -> Self {
        HTTPHeader {
            header_map: HeaderMap::new(),
        }
    }

    pub fn add_field(&mut self, key: &str, value: &str) {
        // TODO: Some nicely done error handling would be welcome here.
        let key = HeaderName::from_str(key).unwrap();
        let value = HeaderValue::from_str(value).unwrap();

        self.header_map.insert(key, value);
    }
}

impl Into<HeaderMap> for HTTPHeader {
    fn into(self) -> HeaderMap {
        self.header_map
    }
}
