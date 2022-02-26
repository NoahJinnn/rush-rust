use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

fn thread_demo() {
    // Create thread
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..2 {
        println!("number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap(); // wait until all thread finish

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    }); // use "move" so spawned thread can capture main thread data
    handle.join().unwrap();
}

fn channel_demo() {
    // Create channel, mpsc is "multiple producer"
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap(); // block main thread exec & wait until a value is sent down
    println!("Got: {}", received);

    // send multiple data down to channel
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn mutex_demo() {
    // Implement multiple threads access same mutext and inc value
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
