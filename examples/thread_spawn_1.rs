use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::Builder::new()
        .name("child_thread".to_string())
        .spawn(|| {
            for i in 1..10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1));
                if i == 10 {
                    panic!("omg!")
                }
            }
        })
        .expect("failed to spawn thread");

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    let _ = handle.join();
}
