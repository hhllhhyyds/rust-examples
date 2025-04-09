use tokio::runtime::Runtime;
use tokio::sync::mpsc;

use std::pin::{Pin, pin};
use std::time::Duration;

fn main() {
    let rt = Runtime::new().unwrap();

    let future = async {
        let (tx, mut rx) = mpsc::unbounded_channel();
        let tx1 = tx.clone();

        // let tx_fut1 = async move {
        //     let vals = vec![
        //         String::from("hi"),
        //         String::from("from"),
        //         String::from("the"),
        //         String::from("future"),
        //     ];

        //     for val in vals {
        //         tx.send(val).unwrap();
        //         tokio::time::sleep(Duration::from_millis(500)).await;
        //     }
        // };

        // let tx_fut2 = async move {
        //     let vals = vec![
        //         String::from("another"),
        //         String::from("message"),
        //         String::from("of"),
        //         String::from("fut2"),
        //     ];

        //     for val in vals {
        //         tx1.send(val).unwrap();
        //         tokio::time::sleep(Duration::from_millis(500)).await;
        //     }
        // };

        // let rx_fut = async {
        //     while let Some(value) = rx.recv().await {
        //         println!("received '{value}'");
        //     }
        // };

        // let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
        //     vec![Box::pin(tx_fut1), Box::pin(tx_fut2), Box::pin(rx_fut)];

        let tx_fut1 = pin!(async move {
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
        });

        let tx_fut2 = pin!(async move {
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
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx_fut1, tx_fut2, rx_fut];

        futures_util::future::join_all(futures).await;
    };

    rt.block_on(future);
}
