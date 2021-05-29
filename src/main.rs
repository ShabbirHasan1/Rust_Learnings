// use std::collections::HashMap;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let resp = reqwest::get("https://httpbin.org/ip")
//         .await?
//         .json::<HashMap<String, String>>()
//         .await?;
//     println!("{:#?}", resp);
//     Ok(())
// }

#![deny(warnings)]
use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, AUTHORITY, COOKIE, USER_AGENT,
};

// This is using the `tokio` runtime. You'll need the following dependency:
//
// `tokio = { version = "1", features = ["full"] }`
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut headers = HeaderMap::new();
    // add the user-agent header required by github
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 11_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.77 Safari/537.36"));
    headers.insert(AUTHORITY, HeaderValue::from_static("www.nseindia.com"));
    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"));
    headers.insert(COOKIE, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 11_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.77 Safari/537.36"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-IN,en;q=0.9"));
    // let res = reqwest::get("https://api.sensibull.com/v1/fii_dii_details").await?;
    let res = reqwest::Client::new()
        .get("https://www.nseindia.com/api/chart-databyindex?index=NIFTY%2050&indices=true")
        .headers(headers)
        .send()
        .await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    println!("Body:\n\n{}", body);

    Ok(())
}
fn main() {}
