use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use super::{extract::extract_between, filter::is_forbidden_ip};

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
