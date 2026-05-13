use std::collections::HashMap;
use std::error::Error;

use zbus::{Connection, zvariant::Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection = Connection::session().await?;

    let m = connection
        .call_method(
            Some("org.freedesktop.Notifications"),
            "/org/freedesktop/Notifications",
            Some("org.freedesktop.Notifications"),
            "Notify",
            &(
                "notification from zbus",
                0u32,
                "notification from zbus",
                "Hello from zbus",
                "Hello from zbus",
                vec![""; 0],
                HashMap::<&str, &Value>::new(),
                5000,
            ),
        )
        .await?;

    let reply: u32 = m.body().deserialize().unwrap();
    dbg!(reply);
    Ok(())
}
