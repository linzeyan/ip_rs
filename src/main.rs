use clap::{ArgAction, Parser};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use scraper::{Html, Selector};
use serde_json::{to_string_pretty, Value};
use std::error::Error;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Sets the URL to request
    #[arg(short, long, default_value = "https://myip.ruru910.com")]
    url: Option<String>,

    /// Print full informations
    #[arg(long, action = ArgAction::SetTrue)]
    full: Option<bool>,

    /// Specifies a JSON field to extract
    #[arg(short, long, default_value = "IP")]
    field: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // get content
    let content = match do_request(args.url.as_deref()).await {
        Ok(content) => content,
        Err(err) => {
            return Err(err);
        }
    };

    // process content
    process_content(&content, args)?;
    Ok(())
}

// do_request
async fn do_request(url: Option<&str>) -> Result<String, Box<dyn Error>> {
    // create client
    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .timeout(Duration::new(3, 0))
        .build()?;

    // create headers
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("rust-agent"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // send request
    let response = client.get(url.unwrap()).headers(headers).send().await?;

    // check status code
    if response.status() != reqwest::StatusCode::OK {
        println!("http error: {}", response.status());
        return Err(format!("Unexpected HTTP code: {}", response.status()).into());
    }
    // return response body as string
    Ok(response.text().await?)
}

// process_content
fn process_content(content: &str, args: Args) -> Result<(), Box<dyn Error>> {
    let field = args.field.as_deref();
    let full = args.full.unwrap();
    match serde_json::from_str::<Value>(content) {
        Ok(json) => {
            if full {
                println!("{}", to_string_pretty(&json)?);
            } else {
                match field {
                    Some(f) => match json.get(f) {
                        Some(value) => {
                            if let Some(ip_str) = value.as_str() {
                                println!("{}", ip_str);
                            } else {
                                println!("{}", value);
                            }
                        }
                        None => println!("{}", to_string_pretty(&json)?),
                    },
                    None => println!("{}", to_string_pretty(&json)?),
                }
            }
        }
        Err(_) => {
            let fragment = Html::parse_document(content);
            let body_selector = Selector::parse("body").unwrap();

            let body_content = fragment
                .select(&body_selector)
                .next() // select first <body> element
                .map(|body| body.text().collect::<Vec<_>>().join(" ")) // remove tags
                .unwrap_or_else(|| "No body content found".to_string());

            // 印出 <body> 中的文本內容
            println!("{}", body_content);
        }
    }
    Ok(())
}
