use futures_util::stream::TryStreamExt;
use zbus::{Connection, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let connection = Connection::session().await?;
    let mut stream = zbus::MessageStream::from(&connection);
    connection.request_name("org.zbus.Greeter").await?;

    while let Some(msg) = stream.try_next().await? {
        let msg_header = msg.header();
        dbg!(&msg);

        match msg_header.message_type() {
            zbus::message::Type::MethodCall => {
                let header = msg.header();
                let body = msg.body();
                let arg: &str = body.deserialize()?;
                connection
                    .reply(&header, &(format!("Hello {}!", arg)))
                    .await?;

                // break;
            }
            _ => continue,
        }
    }

    Ok(())
}
