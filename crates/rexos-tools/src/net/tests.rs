use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use reqwest::Url;
use rexos_kernel::security::{EgressConfig, EgressRule, SecurityConfig};

use super::{extract::extract_between, filter::is_forbidden_ip, policy::egress_rule_allows};

#[test]
fn extract_between_returns_slice_between_markers() {
    assert_eq!(
        extract_between("prefix [value] suffix", "[", "]"),
        Some("value")
    );
}

#[test]
fn extract_between_returns_none_when_marker_missing() {
    assert_eq!(extract_between("prefix", "[", "]"), None);
    assert_eq!(extract_between("prefix [value", "[", "]"), None);
}

#[test]
fn is_forbidden_ip_blocks_carrier_grade_nat_and_site_local_ranges() {
    assert!(is_forbidden_ip(IpAddr::V4(Ipv4Addr::new(100, 64, 0, 1))));
    assert!(is_forbidden_ip(IpAddr::V6(Ipv6Addr::new(
        0xfec0, 0, 0, 0, 0, 0, 0, 1
    ))));
}

#[test]
fn is_forbidden_ip_allows_public_addresses() {
    assert!(!is_forbidden_ip(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1))));
    assert!(!is_forbidden_ip(IpAddr::V6(Ipv6Addr::new(
        0x2606, 0x4700, 0, 0, 0, 0, 0, 0x1111,
    ))));
}

#[test]
fn egress_rule_allows_exact_host_path_and_method_match() {
    let url = Url::parse("https://docs.rs/serde").unwrap();
    let config = SecurityConfig {
        egress: EgressConfig {
            rules: vec![EgressRule {
                tool: "web_fetch".to_string(),
                host: "docs.rs".to_string(),
                path_prefix: "/serde".to_string(),
                methods: vec!["GET".to_string()],
            }],
        },
        ..Default::default()
    };

    assert!(egress_rule_allows("web_fetch", "GET", &url, &config).is_ok());
}

#[test]
fn egress_rule_rejects_method_mismatch() {
    let url = Url::parse("https://docs.rs/serde").unwrap();
    let config = SecurityConfig {
        egress: EgressConfig {
            rules: vec![EgressRule {
                tool: "web_fetch".to_string(),
                host: "docs.rs".to_string(),
                path_prefix: "/serde".to_string(),
                methods: vec!["POST".to_string()],
            }],
        },
        ..Default::default()
    };

    let err = egress_rule_allows("web_fetch", "GET", &url, &config).unwrap_err();
    assert!(err.to_string().contains("method"), "{err}");
}

#[test]
fn egress_rule_rejects_unknown_host() {
    let url = Url::parse("https://example.com/serde").unwrap();
    let config = SecurityConfig {
        egress: EgressConfig {
            rules: vec![EgressRule {
                tool: "web_fetch".to_string(),
                host: "docs.rs".to_string(),
                path_prefix: "/".to_string(),
                methods: vec!["GET".to_string()],
            }],
        },
        ..Default::default()
    };

    let err = egress_rule_allows("web_fetch", "GET", &url, &config).unwrap_err();
    assert!(err.to_string().contains("host"), "{err}");
}

#[test]
fn egress_rule_rejects_empty_rule_set() {
    let url = Url::parse("https://docs.rs/serde").unwrap();
    let err = egress_rule_allows("web_fetch", "GET", &url, &SecurityConfig::default()).unwrap_err();
    assert!(
        err.to_string().contains("egress") || err.to_string().contains("rule"),
        "{err}"
    );
}
