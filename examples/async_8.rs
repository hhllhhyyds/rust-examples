use std::{pin::pin, time::Duration};
use tokio::runtime::Runtime;
use tokio_stream::{Stream, StreamExt, wrappers::UnboundedReceiverStream};

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

    tokio::task::spawn(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            tokio::time::sleep(Duration::from_millis(time_to_sleep)).await;

            tx.send(format!("Message: '{message}'")).unwrap();
        }
    });

    UnboundedReceiverStream::new(rx)
}

fn main() {
    let rt = Runtime::new().unwrap();

    let future = async {
        let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    };

    rt.block_on(future);
}
