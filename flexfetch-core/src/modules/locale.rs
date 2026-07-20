use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;

pub struct LocaleModule;

impl Module for LocaleModule {
    fn name(&self) -> &'static str {
        "locale"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let lang = std::env::var("LANG").unwrap_or_default();
        let encoding = std::env::var("LC_CTYPE")
            .or_else(|_| std::env::var("LC_ALL"))
            .unwrap_or_default();

        let mut map = HashMap::new();
        if !lang.is_empty() {
            map.insert("lang".into(), lang);
        }
        if !encoding.is_empty() {
            map.insert("encoding".into(), encoding);
        }

        if map.is_empty() {
            Ok(InfoValue::Scalar("unknown".into()))
        } else {
            Ok(InfoValue::Map(map))
        }
    }
}
