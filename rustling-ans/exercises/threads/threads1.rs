// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)


use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status1 = Arc::clone(&status);
    let status2 = Arc::clone(&status);
    let handle = thread::spawn(move || {
        for _ in 0..10 {
            let mut shared = status1.lock().unwrap();
            thread::sleep(Duration::from_millis(250));
            shared.jobs_completed += 1;
        }
    });
    while status2.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
    handle.join().unwrap();
}
