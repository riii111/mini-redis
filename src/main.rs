use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listner = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listner.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(socket: TcpStream) {
    // The 'Connection' lets us read/write redis **frames** instead of byte streams.
    // The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        // Respond with an error
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}
