use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct NanoError {
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct Login {
    pub auth_token: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Response {
    pub data: Data,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Data {
    #[serde(rename = "groups")]
    Group(Object<Group>),
}

#[derive(Debug, Deserialize)]
pub struct Object<T>
where
    T: std::fmt::Debug,
{
    pub id: String,
    pub links: HashMap<String, String>,
    pub attributes: T,
    pub relationships: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct Group {
    pub name: String,
}
