use std::time::Duration;

fn main() {
    trpl::block_on(async { message_passing_between_tasks().await });
}

async fn message_passing_between_tasks() {
    let (sender, mut receiver) = trpl::channel();
    let sender1 = sender.clone();

    let sender_future = async move {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            sender.send(val).unwrap();
            trpl::sleep(Duration::from_millis(1500)).await;
        }
    };

    let sender1_future = async move {
        let values = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for value in values {
            sender1.send(value).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let receiver_future = async {
        while let Some(value) = receiver.recv().await {
            println!("received '{value}'");
        }
    };

    trpl::join!(sender1_future, sender_future, receiver_future);
}
