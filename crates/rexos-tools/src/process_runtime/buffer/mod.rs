mod push;
mod text;

#[derive(Debug, Default)]
pub(crate) struct ProcessOutputBuffer {
    head: Vec<u8>,
    tail: Vec<u8>,
    total_bytes: usize,
}

impl ProcessOutputBuffer {
    pub(crate) fn push(&mut self, chunk: &[u8]) {
        push::push_chunk(self, chunk)
    }

    pub(crate) fn take_text(&mut self) -> (String, bool) {
        text::take_text(self)
    }

    #[cfg(test)]
    pub(crate) fn reconstruct_all_bytes(&self) -> Vec<u8> {
        text::reconstruct_all_bytes(self)
    }
}
