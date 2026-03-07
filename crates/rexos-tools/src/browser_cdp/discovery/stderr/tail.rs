use std::collections::VecDeque;

const MAX_TAIL_LINES: usize = 20;
const MAX_LINE_CHARS: usize = 500;

pub(super) fn stderr_tail() -> VecDeque<String> {
    VecDeque::new()
}

pub(super) fn push_stderr_line(tail: &mut VecDeque<String>, line: String) {
    let mut line = line.trim_end().to_string();
    if line.len() > MAX_LINE_CHARS {
        line.truncate(MAX_LINE_CHARS);
        line.push_str("...");
    }
    tail.push_back(line);
    if tail.len() > MAX_TAIL_LINES {
        tail.pop_front();
    }
}

pub(super) fn timeout_error(tail: &VecDeque<String>, prefix: &str) -> String {
    let tail = tail.iter().cloned().collect::<Vec<_>>().join("\n");
    if tail.is_empty() {
        return prefix.to_string();
    }
    format!("{prefix}\n\nChromium stderr (tail):\n{tail}")
}
