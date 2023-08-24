// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::AwsRegionalProductServices;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: AwsRegionalProductServices = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsRegionalProductServices {
    pub metadata: Metadata,

    pub prices: Vec<Price>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub copyright: String,

    pub disclaimer: String,

    #[serde(rename = "format:version")]
    pub format_version: String,

    #[serde(rename = "source:version")]
    pub source_version: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Price {
    pub attributes: Attributes,

    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attributes {
    #[serde(rename = "aws:region")]
    pub aws_region: String,

    #[serde(rename = "aws:serviceName")]
    pub aws_service_name: String,

    #[serde(rename = "aws:serviceUrl")]
    pub aws_service_url: String,
}
