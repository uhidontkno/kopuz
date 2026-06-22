use std::sync::OnceLock;

use regex::Regex;

static BEARER_TOKEN: OnceLock<String> = OnceLock::new();

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

pub async fn get_bearer_token() -> Result<String, String> {
    if let Some(t) = BEARER_TOKEN.get() {
        return Ok(t.clone());
    }

    let client = reqwest::Client::new();
    let resp = client
        .get("https://music.apple.com")
        .header("User-Agent", USER_AGENT)
        .send()
        .await
        .map_err(|e| format!("fetch music.apple.com: {e}"))?;

    let body = resp
        .text()
        .await
        .map_err(|e| format!("read music.apple.com body: {e}"))?;

    let js_re = Regex::new(r"/assets/index~[^/]+\.js").unwrap();
    let js_path = js_re
        .find(&body)
        .ok_or("no index~*.js found on music.apple.com")?
        .as_str();

    let js_resp = client
        .get(format!("https://music.apple.com{js_path}"))
        .header("User-Agent", USER_AGENT)
        .send()
        .await
        .map_err(|e| format!("fetch JS bundle: {e}"))?;

    let js_body = js_resp
        .text()
        .await
        .map_err(|e| format!("read JS bundle: {e}"))?;

    let jwt_re = Regex::new(r"eyJ[A-Za-z0-9\-_]+\.[A-Za-z0-9\-_]+\.[A-Za-z0-9\-_]+").unwrap();
    let token = jwt_re
        .find(&js_body)
        .ok_or("no Bearer JWT found in JS bundle")?
        .as_str()
        .to_string();

    let _ = BEARER_TOKEN.set(token.clone());
    Ok(token)
}
