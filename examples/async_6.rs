use tokio::runtime::Runtime;

use futures_util::future::Either;

use std::pin::pin;
use std::time::Duration;

fn slow(name: &str, ms: u64) {
    std::thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

fn main() {
    let rt = Runtime::new().unwrap();

    let future = async {
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            tokio::task::yield_now().await;
            slow("a", 10);
            tokio::task::yield_now().await;
            slow("a", 20);
            tokio::time::sleep(Duration::from_millis(50)).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            tokio::task::yield_now().await;
            slow("b", 10);
            tokio::task::yield_now().await;
            slow("b", 15);
            tokio::task::yield_now().await;
            slow("b", 350);
            tokio::time::sleep(Duration::from_millis(50)).await;
            println!("'b' finished.");
        };

        let f1 = pin!(a);
        let f2 = pin!(b);

        match futures_util::future::select(f1, f2).await {
            Either::Left((a, _f2)) => Either::Left(a),
            Either::Right((b, _f1)) => Either::Right(b),
        };

        let one_ns = Duration::from_nanos(1);
        let start = std::time::Instant::now();
        async {
            for _ in 1..1000 {
                tokio::time::sleep(one_ns).await;
            }
        }
        .await;
        let time = std::time::Instant::now() - start;
        println!(
            "'sleep' version finished after {} seconds.",
            time.as_secs_f32()
        );

        let start = std::time::Instant::now();
        async {
            for _ in 1..1000 {
                tokio::task::yield_now().await;
            }
        }
        .await;
        let time = std::time::Instant::now() - start;
        println!(
            "'yield' version finished after {} seconds.",
            time.as_secs_f32()
        );
    };

    rt.block_on(future);
}
