mod permissions;
mod tool_gate;

pub(crate) use permissions::{skill_approval_is_granted, skill_permissions_are_readonly};

#[cfg(test)]
mod tests {
    use super::permissions::skill_permissions_are_readonly;
    use super::tool_gate::tool_requires_approval;

    #[test]
    fn readonly_permissions_ignore_safe_entries() {
        let permissions = vec!["readonly".to_string(), "tool:file_read".to_string()];
        assert!(skill_permissions_are_readonly(&permissions));
    }

    #[test]
    fn readonly_permissions_reject_write_like_entries() {
        let permissions = vec!["tool:apply_patch".to_string()];
        assert!(!skill_permissions_are_readonly(&permissions));
    }

    #[test]
    fn approval_is_required_for_risky_tools_or_private_network_access() {
        assert!(tool_requires_approval("shell", "{}", false));
        assert!(tool_requires_approval(
            "browser_navigate",
            r#"{"allow_private":true}"#,
            false,
        ));
        assert!(!tool_requires_approval(
            "browser_navigate",
            r#"{"allow_private":false}"#,
            false
        ));
    }
}
