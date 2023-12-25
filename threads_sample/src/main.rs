use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::{thread, vec};

fn main() {
    //example_1()
    // example_2_join();

    //  example_3_closure();
    //  example_4_passing_message();

    //  example_5_passing__multi_message_single_producer();

    //  example_6_passing__multi_message_multi_producer();
    // example_7_sharing_state();

    example_8_sharing_state_multi_thread();
}

/// .
fn example_1() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(std::time::Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(std::time::Duration::from_millis(1));
    }
}

fn example_2_join() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(std::time::Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(std::time::Duration::from_millis(1));
    }

    //  handle.join().unwrap();
}

fn example_3_closure() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Current Vector :{:?}", v);
    });

    handle.join().unwrap();
}

fn example_4_passing_message() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("hi");
        tx.send(msg).unwrap();
    });

    let received = rx.recv().unwrap();

    println!("Got: {}", received);
}

fn example_5_passing__multi_message_single_producer() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for msg in vals {
            tx.send(msg).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn example_6_passing__multi_message_multi_producer() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for msg in vals {
            tx.send(msg).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi 2"),
            String::from("from 2"),
            String::from("the 2"),
            String::from("thread 2"),
        ];

        for msg in vals {
            tx1.send(msg).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn example_7_sharing_state() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    print!("m = {:?}", m);
}

fn example_8_sharing_state_multi_thread() {
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
