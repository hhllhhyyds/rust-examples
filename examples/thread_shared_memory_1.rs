use std::ops::Range;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    let start = std::time::Instant::now();

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs_f32(
                rand::random_range::<f32, Range<f32>>(0.0..3.0),
            ));
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("thread {i} add 1 to num, num = {num}");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let time_used = std::time::Instant::now() - start;

    println!(
        "Result: {}, time used = {} s",
        *counter.lock().unwrap(),
        time_used.as_secs_f32()
    );
}
