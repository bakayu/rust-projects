use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use mini_redis::Command::{self, Get, Set};
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

/// A sharded in-memory key/value store.
///
/// This is an **in-process, in-memory** store where the full dataset is broken
/// across multiple independent shards. Each shard is a `Mutex<HashMap<...>>`.
///
/// ## Why shard?
///
/// A single `Mutex<HashMap<...>>` (as in `Db`) becomes a scalability bottleneck
/// once you have multiple concurrent tasks/clients. Every operation must lock
/// the same global mutex, even if they access different keys.
///
/// Sharding reduces lock contention by splitting the keyspace into multiple
/// independent buckets. Each bucket has its own `Mutex`, so concurrent reads/writes
/// to different shards can proceed in parallel.
///
/// ## How it works
///
/// 1. A hash of the key is computed (via `std::hash::Hash` / `DefaultHasher`).
/// 2. The hash is mapped to a shard index via `hash % num_shards`.
/// 3. Only the chosen shard’s `Mutex` is locked for the duration of the operation.
///
/// This keeps locking fine-grained while still using a simple `Mutex` for each
/// shard (no complex lock-free structures required).
///
/// ## Example
///
/// ```rust
/// let db = new_sharded_db(16); // 16 shards
///
/// // Set a value (locks only one shard)
/// {
///     let mut shard = shard_for(&db, "my_key").lock().unwrap();
///     shard.insert("my_key".to_string(), Bytes::from("value"));
/// }
///
/// // Get a value (locks only one shard)
/// {
///     let shard = shard_for(&db, "my_key").lock().unwrap();
///     let value = shard.get("my_key");
/// }
/// ```
///
/// ## When to use
///
/// - Use sharding when you expect many concurrent operations and want to avoid
///   global mutex contention.
/// - If the workload is low concurrency, a single `Mutex<HashMap<...>>` is simpler.
///
/// **Note:** Sharding only reduces contention; it does not change correctness
/// semantics (it’s still a global store, just split into independent buckets).
type ShardedDb = Arc<Vec<Mutex<HashMap<String, Bytes>>>>;
#[allow(dead_code)]
type Db = Arc<Mutex<HashMap<String, Bytes>>>;

/// Create a new `ShardedDb` with a number of shards
fn new_sharded_db(num_shards: usize) -> ShardedDb {
    let mut db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}

/// Get Mutex of the destination shard from a ShardedDb
fn shard_for<'a, K: Hash + ?Sized>(db: &'a ShardedDb, key: &K) -> &'a Mutex<HashMap<String, Bytes>>
where
    K: Hash,
{
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    let idx = (hasher.finish() % db.len() as u64) as usize;
    &db[idx]
}

/// Process incoming messages from clients and perform `Set`/`Get` operations
async fn process(socket: TcpStream, db: ShardedDb) {
    let mut connection = Connection::new(socket);

    // use read_frame to receive a command from the connection
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db_shard = shard_for(&db, cmd.key()).lock().unwrap();
                db_shard.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db_shard = shard_for(&db, cmd.key()).lock().unwrap();
                if let Some(value) = db_shard.get(cmd.key()) {
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
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening on: 127.0.0.1:6379");

    let db = new_sharded_db(16);

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();

        println!("Accepted");

        tokio::spawn(async move { process(socket, db).await });
    }
}
