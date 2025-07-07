use napi::bindgen_prelude::*;
use napi_derive::napi;
use serde_json::Value;
use json_stream_parser::{parse_stream as parse_stream_inner, JsonStreamParser};

#[napi]
pub fn parse_stream(input: String) -> Result<Value> {
    parse_stream_inner(&input).map_err(|e| Error::new(Status::GenericFailure, e))
}

#[napi]
pub struct JsJsonStreamParser {
    inner: JsonStreamParser,
}

#[napi]
impl JsJsonStreamParser {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            inner: JsonStreamParser::new(),
        }
    }

    #[napi]
    pub fn add_chunk(&mut self, chunk: String) -> Result<()> {
        for ch in chunk.chars() {
            self.inner
                .add_char(ch)
                .map_err(|e| Error::new(Status::GenericFailure, e.clone()))?;
        }
        Ok(())
    }

    #[napi]
    pub fn get_result(&self) -> Value {
        self.inner.get_result().clone()
    }
}

