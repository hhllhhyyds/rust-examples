use tokio::runtime::Runtime;
use tokio::sync::mpsc;

use std::time::Duration;

fn main() {
    let rt = Runtime::new().unwrap();

    let future = async {
        let (tx, mut rx) = mpsc::unbounded_channel();
        let tx1 = tx.clone();

        let tx_fut1 = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        };

        let tx_fut2 = async move {
            let vals = vec![
                String::from("another"),
                String::from("message"),
                String::from("of"),
                String::from("fut2"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        futures_util::future::join3(tx_fut1, tx_fut2, rx_fut).await;
    };

    rt.block_on(future);
}
