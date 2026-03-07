mod percent;
mod redirect;
#[cfg(test)]
mod tests;

pub(super) fn decode_result_url(url: String) -> String {
    if let Some(target) = redirect::duckduckgo_redirect_target(&url) {
        percent::decode_url_component(target)
    } else {
        url
    }
}
