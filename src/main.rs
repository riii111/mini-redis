use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

mod command;
use command::Command;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let manager = tokio::spawn(async move {
        let mut client = match client::connect("127.0.0.1:6379").await {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to connect to Redis server: {}", e);
                return;
            }
        };
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;
                    let _ = resp.send(res); // ignore error
                }
                Command::Set { key, val, resp } => {
                    let res = client.set(&key, val.into()).await;
                    let _ = resp.send(res); // ignore error
                }
            }
        }
    });

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "hello".to_string(),
            resp: resp_tx,
        };

        if tx.send(cmd).await.is_err() {
            eprintln!("connection task shutdown");
            return;
        }

        let res = resp_rx.await;
        println!("GOT (Get) = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: b"bar".to_vec(),
            resp: resp_tx,
        };

        if tx2.send(cmd).await.is_err() {
            eprintln!("connection task shutdown");
            return;
        }

        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
