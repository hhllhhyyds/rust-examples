use tokio::runtime::Runtime;

use futures_util::future::Either;

use std::pin::pin;
use std::time::Duration;

use scraper::{Html, Selector};

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = reqwest::get(url).await.unwrap().text().await.unwrap();

    let title = Html::parse_document(&response_text)
        .select(&Selector::parse("title").unwrap())
        .nth(0)
        .map(|title_element| title_element.inner_html());

    (url, title)
}

async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    let time_future = tokio::time::sleep(max_time);

    let f1 = pin!(future_to_try);
    let f2 = pin!(time_future);

    match futures_util::future::select(f1, f2).await {
        Either::Left((a, _f2)) => Ok(a),
        Either::Right(_) => Err(max_time),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let rt = Runtime::new().unwrap();

    let future = async {
        loop {
            let get_title_fut = page_title(&args[1]);

            match timeout(get_title_fut, Duration::from_secs_f32(0.1)).await {
                Ok((url, title)) => {
                    println!("Successfully get title of {url} with title = '{title:?}'");
                    break;
                }
                Err(duration) => {
                    println!(
                        "Failed to get title after {} seconds, retry",
                        duration.as_secs_f32()
                    )
                }
            }
        }
    };

    rt.block_on(future);
}
