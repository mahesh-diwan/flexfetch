use crate::{Context, InfoValue, Module, Result};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

pub struct PublicIpModule;

impl Module for PublicIpModule {
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        // Phase 4.1: reuse the shared cache (60 s TTL) so repeated invocations
        // skip the ~0.5 s network round-trip entirely.
        if let Ok(cache) = ctx.cache.lock() {
            if let Some(ip) = cache.get("publicip") {
                return Ok(InfoValue::Scalar(ip));
            }
        }

        match fetch_public_ip() {
            Some(ip) if !ip.is_empty() => {
                if let Ok(mut cache) = ctx.cache.lock() {
                    cache.set("publicip", ip.clone());
                }
                Ok(InfoValue::Scalar(ip))
            }
            _ => Ok(InfoValue::Scalar("unknown".into())),
        }
    }
}

/// Phase 4.1: fetch the public IPv4 over a bare TCP HTTP/1.1 GET to
/// api.ipify.org — no `curl` subprocess (zero-spawn). A 2 s read timeout keeps
/// the module from stalling the fetch when the network is slow or down.
fn fetch_public_ip() -> Option<String> {
    let mut stream = TcpStream::connect(("api.ipify.org", 80)).ok()?;
    stream.set_read_timeout(Some(Duration::from_secs(2))).ok()?;
    write!(
        stream,
        "GET / HTTP/1.1\r\nHost: api.ipify.org\r\nConnection: close\r\n\r\n"
    )
    .ok()?;
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).ok()?;
    let text = String::from_utf8_lossy(&buf);
    // Body follows the blank line after the headers.
    let body = text.split("\r\n\r\n").nth(1)?;
    let ip = body.trim().to_string();
    if ip.is_empty() {
        None
    } else {
        Some(ip)
    }
}
