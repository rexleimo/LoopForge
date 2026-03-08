use crate::release_check::ReleaseCheckReport;

pub(crate) fn format_release_check_report(report: &ReleaseCheckReport) -> String {
    let mut out = String::new();
    out.push_str(&format!("Release check for {}\n\n", report.tag));
    for check in &report.checks {
        let prefix = if check.ok { "OK  " } else { "ERR " };
        out.push_str(&format!("{prefix} {}: {}\n", check.id, check.message));
    }
    out.push_str(&format!(
        "\nSummary: {}\n",
        if report.ok { "PASS" } else { "FAIL" }
    ));
    out
}
