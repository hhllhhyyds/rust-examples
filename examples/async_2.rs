use std::time::Duration;

use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();

    let future = async {
        let handle = tokio::task::spawn(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            tokio::time::sleep(Duration::from_millis(500)).await;
        }

        handle.await.unwrap();
    };

    rt.block_on(future);
}
