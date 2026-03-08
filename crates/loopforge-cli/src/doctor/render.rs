use super::{CheckStatus, DoctorReport};

impl DoctorReport {
    pub fn exit_code(&self, strict: bool) -> i32 {
        if self.summary.error > 0 {
            return 1;
        }
        if strict && self.summary.warn > 0 {
            return 1;
        }
        0
    }

    pub fn to_text(&self) -> String {
        let mut out = String::new();
        out.push_str("LoopForge doctor\n\n");
        for check in &self.checks {
            let prefix = match check.status {
                CheckStatus::Ok => "OK  ",
                CheckStatus::Warn => "WARN",
                CheckStatus::Error => "ERR ",
            };
            if check.message.trim().is_empty() {
                out.push_str(&format!("{prefix} {id}\n", id = check.id));
            } else {
                out.push_str(&format!(
                    "{prefix} {id}: {msg}\n",
                    id = check.id,
                    msg = check.message
                ));
            }
        }
        out.push_str(&format!(
            "\nSummary: ok={} warn={} error={}\n",
            self.summary.ok, self.summary.warn, self.summary.error
        ));
        if !self.next_actions.is_empty() {
            out.push_str("\nSuggested next steps:\n");
            for action in &self.next_actions {
                out.push_str(&format!("- {action}\n"));
            }
        }
        out
    }
}
