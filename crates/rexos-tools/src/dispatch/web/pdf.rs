use crate::defs::PdfArgs;
use crate::Toolset;

pub(super) async fn dispatch(toolset: &Toolset, arguments_json: &str) -> anyhow::Result<String> {
    let args: PdfArgs = super::super::parse_args(arguments_json, "pdf")?;
    toolset
        .pdf_extract(
            &args.path,
            args.pages.as_deref(),
            args.max_pages,
            args.max_chars,
        )
        .await
}
