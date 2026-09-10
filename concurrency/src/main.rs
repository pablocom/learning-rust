use std::{sync::mpsc, thread, time::Duration};

fn main() {
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
