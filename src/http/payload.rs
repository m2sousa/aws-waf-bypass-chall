extern crate serde;

use serde::Serialize;
use serde_json::{Map, Value as Json};
use std::collections::HashMap;

pub struct HTTPPayload<'a> {
    fields_map: HashMap<&'a str, &'a str>,
}

impl<'a> HTTPPayload<'a> {
    pub fn new() -> Self {
        HTTPPayload {
            fields_map: HashMap::new(),
        }
    }

    pub fn add_field(&mut self, key: &'a str, value: &'a str) {
        self.fields_map.insert(key, value);
    }

    pub(super) fn parse_json(&self) -> Json {
        let mut map = Map::new();

        for (&k, &v) in &self.fields_map {
            let value = if v.starts_with('{') || v.starts_with('[') {
                serde_json::from_str::<Json>(v).unwrap_or(Json::String(v.to_string()))
            } else {
                Json::String(v.to_string())
            };

            map.insert(k.to_string(), value);
        }

        Json::Object(map)
    }
}

impl<'a> Serialize for HTTPPayload<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.fields_map.serialize(serializer)
    }
}
