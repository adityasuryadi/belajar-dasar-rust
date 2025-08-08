use std::{sync::Arc, thread, time::Duration};

use tokio::spawn;
use tokio::{sync::Notify, task, time::sleep};
#[tokio::test]

async fn test_notify() {
    let notify = Arc::new(Notify::new());
    let notify_clone = notify.clone();

    tokio::spawn(async move {
        println!("Task 1: doing Some work...");
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        println!("Task 1:Notifying others");
        notify_clone.notify_one();
    });

    println!("Main: Waiting for notification");
    notify.notified().await;
    println!("Main: Got Notified");
}

#[tokio::test]
async fn test_notify2() {
    let notify = Arc::new(Notify::new());
    let mut handlers = vec![];

    // buat beberapa consumer
    for i in 0..5 {
        let notify = notify.clone();
        handlers.push(tokio::spawn(async move {
            // tunggu jika ada task yang lain
            notify.notified().await;
            println!("Customer {} got notified", i);
        }));
    }

    println!("start");
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    println!("producer send notification");
    notify.notify_waiters();

    for handler in handlers {
        handler.await.unwrap();
    }
}
