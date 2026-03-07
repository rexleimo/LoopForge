use rexos_llm::openai_compat::ToolDefinition;

use super::super::shared::{function_def, pdf_parameters};

pub(super) fn pdf_def() -> ToolDefinition {
    function_def(
        "pdf",
        "Extract text from a PDF file in the workspace.",
        pdf_parameters(),
    )
}

pub(super) fn pdf_extract_def() -> ToolDefinition {
    function_def(
        "pdf_extract",
        "Alias for `pdf`. Extract text from a PDF file in the workspace.",
        pdf_parameters(),
    )
}
