/// Background Remover by <a href="https://erasebg.org">Erase BG</a>
///
/// Being developed!

pub fn full_url(api_key: &str) -> String {
    return format!("https://api.erasebg.org/v1/remove-background/upload/?api_key={}", api_key);
}
