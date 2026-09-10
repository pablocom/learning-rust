use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
    time::Duration,
};

fn main() {
    channels();
    // mutexes();
    // creating_a_deadlock();
}

#[allow(dead_code)]
fn channels() {
    let (sender, receiver) = mpsc::channel();
    let another_sender = sender.clone();

    thread::spawn(move || {
        let values = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for value in values {
            sender.send(value).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let values = vec![
            String::from("yo"),
            String::from("this"),
            String::from("was"),
            String::from("inspired"),
            String::from("by"),
            String::from("golang?"),
        ];

        for value in values {
            another_sender.send(value).unwrap();
            thread::sleep(Duration::from_millis(1500));
        }
    });

    let mut iteration = 0usize;
    for received in receiver.iter() {
        println!("Channel receival {iteration}: {received}");
        iteration += 1;
    }
}

#[allow(dead_code)]
fn mutexes() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..thread::available_parallelism().unwrap().get() {
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

#[allow(dead_code)]
fn creating_a_deadlock() {
    let memory_a = Arc::new(Mutex::new(0));
    let memory_b = Arc::new(Mutex::new(0));
    let memory_a_clone = Arc::clone(&memory_a);
    let memory_b_clone = Arc::clone(&memory_b);

    let handle_a = thread::spawn(move || {
        let mut number_a = memory_a.lock().unwrap();

        thread::sleep(Duration::from_millis(10));

        let mut number_b = memory_b.lock().unwrap();

        *number_a += 1;
        *number_b += 1;
    });

    let handle_b = thread::spawn(move || {
        let mut number_b = memory_b_clone.lock().unwrap();

        thread::sleep(Duration::from_millis(10));

        let mut number_a = memory_a_clone.lock().unwrap();

        *number_a += 1;
        *number_b += 1;
    });

    handle_a.join().unwrap();
    handle_b.join().unwrap();
}
