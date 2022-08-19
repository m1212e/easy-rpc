use std::io::Read;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub name: String,
    pub types: Vec<String>,
    pub documentation: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub sources: Vec<String>,
    pub role: String,
}

pub fn parse_roles<T: Read>(input: T) -> Result<Vec<Role>, serde_json::Error> {
    serde_json::from_reader(input)
}

pub fn parse_config<T: Read>(input: T) -> Result<Config, serde_json::Error> {
    serde_json::from_reader(input)
}
