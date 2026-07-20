use crate::Context;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InfoValue {
    Scalar(String),
    Map(HashMap<String, String>),
    List(Vec<String>),
    Table(Vec<HashMap<String, String>>),
}

impl InfoValue {
    pub fn scalar(s: impl Into<String>) -> Self {
        InfoValue::Scalar(s.into())
    }

    pub fn is_empty(&self) -> bool {
        match self {
            InfoValue::Scalar(s) => s.is_empty(),
            InfoValue::Map(m) => m.is_empty(),
            InfoValue::List(l) => l.is_empty(),
            InfoValue::Table(t) => t.is_empty(),
        }
    }
}

pub trait Module: Send + Sync {
    fn name(&self) -> &'static str;
    fn collect(&self, ctx: &Context) -> crate::Result<InfoValue>;
}

pub struct SystemInfo {
    pub entries: Vec<(&'static str, InfoValue)>,
}

impl SystemInfo {
    pub fn new() -> Self {
        SystemInfo {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, name: &'static str, value: InfoValue) {
        self.entries.push((name, value));
    }

    pub fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        for (name, value) in &self.entries {
            map.insert(
                name.to_string(),
                serde_json::to_value(value).unwrap_or_default(),
            );
        }
        serde_json::Value::Object(map)
    }
}
