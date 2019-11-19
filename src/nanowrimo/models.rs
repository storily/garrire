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
pub struct ResponseSingle {
    pub data: Data,
    #[serde(default)]
    pub included: Vec<Data>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ResponseList {
    pub data: Vec<Data>,
    #[serde(default)]
    pub included: Vec<Data>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Data {
    #[serde(rename = "groups")]
    Group(Object<Group>),
    #[serde(rename = "users")]
    User(Object<User>),
    #[serde(rename = "project-challenges")]
    ProjectChallenge(Object<ProjectChallenge>),
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
pub struct Object<T>
where
    T: std::fmt::Debug,
{
    pub id: String,
    pub attributes: T,
    #[serde(default)]
    pub links: HashMap<String, String>,
    #[serde(default)]
    pub relationships: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct Group {
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct User {
    pub name: String,
    pub slug: String, // "username"
    pub email: Option<String>,
    pub time_zone: String,
    pub created_at: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProjectChallenge {
    pub project_id: usize,
    pub starts_at: String,
    pub ends_at: String,
    pub challenge_id: usize,
    pub start_count: Option<usize>,
    pub current_count: Option<usize>,
    pub goal: Option<usize>,
    pub unit_type: Option<usize>,
    pub name: String,
    pub nano_event: bool,
    pub latest_count: Option<usize>,
}
