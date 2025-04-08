use futures_util::future::Either;
use scraper::{Html, Selector};
use std::pin::pin;
use tokio::runtime::Runtime;

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = reqwest::get(url).await.unwrap().text().await.unwrap();

    let title = Html::parse_document(&response_text)
        .select(&Selector::parse("title").unwrap())
        .nth(0)
        .map(|title_element| title_element.inner_html());

    (url, title)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let rt = Runtime::new().unwrap();

    let future = async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let f1 = pin!(title_fut_1);
        let f2 = pin!(title_fut_2);

        let race = match futures::future::select(f1, f2).await {
            Either::Left((a, _f2)) => Either::Left(a),
            Either::Right((b, _f1)) => Either::Right(b),
        };

        let (url, maybe_title) = match race {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");

        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    };

    rt.block_on(future)
}
