mod markers;
mod parser;
mod scan;
#[cfg(test)]
mod tests;

pub(super) fn parse_jpeg_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    parser::parse_jpeg_dimensions(bytes)
}
