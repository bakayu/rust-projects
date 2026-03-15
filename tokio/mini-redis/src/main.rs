use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use mini_redis::Command::{self, Get, Set};
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

type ShardedDb = Arc<Mutex<HashMap<String, Bytes>>>;

async fn process(socket: TcpStream, db: ShardedDb) {
    let mut connection = Connection::new(socket);

    // use read_frame to receive a command from the connection
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
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

#[tokio::main]
async fn main() {
    let listner = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening on: 127.0.0.1:6379");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listner.accept().await.unwrap();
        let db = db.clone();

        println!("Accepted");

        tokio::spawn(async move { process(socket, db).await });
    }
}
