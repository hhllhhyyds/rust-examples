use std::{pin::pin, time::Duration};
use tokio::runtime::Runtime;
use tokio_stream::{Stream, StreamExt, wrappers::UnboundedReceiverStream};

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

    tokio::task::spawn(async move {
        let mut count = 0;
        loop {
            tokio::time::sleep(Duration::from_millis(1)).await;
            count += 1;
            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    UnboundedReceiverStream::new(rx)
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

    tokio::task::spawn(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            tokio::time::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }
    });

    UnboundedReceiverStream::new(rx)
}

fn main() {
    let rt = Runtime::new().unwrap();

    let future = async {
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);

        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    };

    rt.block_on(future);
}
