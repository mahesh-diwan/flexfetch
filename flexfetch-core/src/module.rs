use crate::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_round_trip_preserves_entries() {
        // The `--ssh` remote-fetch path depends entirely on
        // to_json -> from_json round-tripping all InfoValue shapes.
        let mut info = SystemInfo::new();
        info.add("scalar", InfoValue::Scalar("value".into()));
        let mut map = HashMap::new();
        map.insert("key".into(), "val".into());
        info.add("map", InfoValue::Map(map));
        info.add("list", InfoValue::List(vec!["a".into(), "b".into()]));
        let mut row = HashMap::new();
        row.insert("col".into(), "cell".into());
        info.add("table", InfoValue::Table(vec![row]));

        let json = info.to_json();
        let back = SystemInfo::from_json(&json).expect("from_json should succeed");
        assert_eq!(back.to_json(), json, "round trip must be lossless");
        assert_eq!(back.entries.len(), 4);
    }

    #[test]
    fn from_json_rejects_non_object() {
        assert!(SystemInfo::from_json(&serde_json::json!([])).is_err());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Default for SystemInfo {
    fn default() -> Self {
        Self::new()
    }
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

    /// Rebuild a `SystemInfo` from the JSON produced by `to_json` (e.g. parsed
    /// from a remote `flexfetch --format json` run over SSH).
    pub fn from_json(value: &serde_json::Value) -> crate::Result<Self> {
        let obj = value
            .as_object()
            .ok_or_else(|| crate::Error::Template("remote output is not a JSON object".into()))?;
        let mut info = SystemInfo::new();
        for (name, val) in obj {
            let parsed = serde_json::from_value::<InfoValue>(val.clone()).map_err(|e| {
                crate::Error::Template(format!("parse remote value for '{name}': {e}"))
            })?;
            // Box the name to a leaked 'static string: the registry keys are
            // 'static but remote module names are dynamic. A few leaked strings
            // per fetch is negligible for a CLI process.
            let leaked: &'static str = Box::leak(name.clone().into_boxed_str());
            info.add(leaked, parsed);
        }
        Ok(info)
    }
}
