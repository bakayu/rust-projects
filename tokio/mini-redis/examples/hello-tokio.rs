use mini_redis::{Result, client};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "tokio"
    client.set("hello", "tokio".into()).await?;

    let response = client.get("hello").await?;

    println!("RESPONSE: {:?}", response);

    Ok(())
}
