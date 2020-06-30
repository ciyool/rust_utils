use reqwest::header;
use reqwest::Client;

async fn new_http_client(
    headers: Option<header::HeaderMap>,
    timeout: Option<u64>,
    use_cookie: bool,
) -> Result<reqwest::Client, reqwest::Error> {
    let mut headers_in = header::HeaderMap::new();
    headers_in.insert(
        header::UPGRADE_INSECURE_REQUESTS,
        header::HeaderValue::from_static("1"),
    );
    headers_in.insert(header::USER_AGENT, header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.122 Safari/537.36"));
    if let Some(headers) = headers {
        headers.iter().for_each(|(name,value)|{
            if headers_in.contains_key(name) {
                headers_in.remove(name);
            }
            headers_in.insert(name, value.into());
        });
    }
    
    let timeout_in= match timeout {
        Some(ts) => {
            ts
        }
        None => {
            20
        }
    };
    
    let client: reqwest::Client = reqwest::Client::builder()
        .default_headers(headers_in)
        .timeout(std::time::Duration::from_secs(timeout_in))
        .cookie_store(use_cookie) //使用cookie
        .build()?;

    Ok(client)
}
