use std::collections::HashMap;

use mini_redis::Command::{self, Get, Set};
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listner = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listner.accept().await.unwrap();
        tokio::spawn(async move { process(socket).await });
    }
}

async fn process(socket: TcpStream) {
    // hashmap for the key value store
    let mut kv_store = HashMap::new();

    let mut connection = Connection::new(socket);

    // use read_frame to receive a command from the connection
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // The value is stored as `Vec<u8>`
                kv_store.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = kv_store.get(cmd.key()) {
                    // `Frame::Bulk` expects data to be of type `Bytes`.
                    // `&Vec<u8>` is coverted to `Bytes` using `into()`
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}
