mod decode;
mod reader;

use std::sync::Arc;

use crate::process_runtime::ProcessOutputBuffer;
use crate::Toolset;

impl Toolset {
    pub(crate) fn spawn_process_output_reader(
        stream: impl tokio::io::AsyncRead + Unpin + Send + 'static,
        buffer: Arc<tokio::sync::Mutex<ProcessOutputBuffer>>,
    ) {
        reader::spawn_process_output_reader(stream, buffer);
    }

    pub(crate) fn decode_process_output(bytes: Vec<u8>) -> String {
        decode::decode_process_output(bytes)
    }
}
