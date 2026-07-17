use std::convert::Infallible;
use std::net::{IpAddr, SocketAddr};

use axum::extract::{ConnectInfo, FromRequestParts};
use axum::http::HeaderMap;
use axum::http::Uri;
use axum::http::request::Parts;
use axum_limit::{Key, StorageKey};

const FALLBACK_CLIENT_IP: &str = "127.0.0.1";
const X_REAL_IP: &str = "x-real-ip";
const X_FORWARDED_FOR: &str = "x-forwarded-for";

/// Rate-limit subject: one bucket per client IP + request path.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClientIpUri {
    ip: String,
    path: String,
}

#[derive(Debug, Clone)]
pub struct ClientIpUriParts {
    ip: String,
    uri: Uri,
}

impl<S> FromRequestParts<S> for ClientIpUriParts
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let peer_ip = parts
            .extensions
            .get::<ConnectInfo<SocketAddr>>()
            .map(|ConnectInfo(addr)| addr.ip());

        Ok(Self {
            ip: resolve_client_ip(&parts.headers, peer_ip),
            uri: parts.uri.clone(),
        })
    }
}

impl Key for ClientIpUri {
    type Extractor = ClientIpUriParts;

    fn from_extractor(parts: &Self::Extractor) -> Self {
        Self {
            ip: parts.ip.clone(),
            path: parts.uri.path().to_string(),
        }
    }
}

impl StorageKey for ClientIpUri {
    fn storage_key(&self) -> String {
        format!("{}|{}", self.ip, self.path)
    }
}

fn resolve_client_ip(headers: &HeaderMap, peer_ip: Option<IpAddr>) -> String {
    if let Some(ip) = header_ip(headers, X_REAL_IP) {
        return normalize_ip(ip);
    }

    if let Some(ip) = forwarded_for_client_ip(headers) {
        return normalize_ip(ip);
    }

    if let Some(ip) = peer_ip {
        return normalize_ip(ip.to_string());
    }

    FALLBACK_CLIENT_IP.to_string()
}

fn header_ip(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn forwarded_for_client_ip(headers: &HeaderMap) -> Option<String> {
    headers
        .get(X_FORWARDED_FOR)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| {
            value
                .split(',')
                .map(str::trim)
                .find(|part| !part.is_empty())
                .map(ToOwned::to_owned)
        })
}

fn normalize_ip(ip: String) -> String {
    if let Ok(IpAddr::V6(v6)) = ip.parse::<IpAddr>() {
        if let Some(v4) = v6.to_ipv4_mapped() {
            return v4.to_string();
        }
    }
    ip
}
