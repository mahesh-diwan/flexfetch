use crate::{Context, InfoValue, Module, Result};

/// flexfetch's own version — handy for screenshots and support threads.
pub struct VersionModule;

impl Module for VersionModule {
    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        Ok(InfoValue::Scalar(env!("CARGO_PKG_VERSION").to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::test_ctx;
    use crate::fs::MockFs;

    #[test]
    fn reports_crate_version() {
        let ctx = test_ctx(MockFs::new());
        let v = VersionModule.collect(&ctx).unwrap();
        assert_eq!(v.summary(), env!("CARGO_PKG_VERSION"));
    }
}
