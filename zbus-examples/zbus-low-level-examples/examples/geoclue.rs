use futures_util::stream::StreamExt;
use zbus::{Connection, proxy, zvariant::ObjectPath};

#[proxy(
    default_service = "org.freedesktop.GeoClue2",
    interface = "org.freedesktop.GeoClue2.Manager",
    default_path = "/org/freedesktop/GeoClue2/Manager"
)]
trait Manager {
    #[zbus(object = "Client")]
    fn get_client(&self);
}

#[proxy(
    default_service = "org.freedesktop.GeoClue2",
    interface = "org.freedesktop.GeoClue2.Client"
)]
trait Client {
    fn start(&self) -> zbus::Result<()>;
    fn stop(&self) -> zbus::Result<()>;

    #[zbus(property)]
    fn set_desktop_id(&mut self, id: &str) -> zbus::Result<()>;

    #[zbus(signal)]
    fn locationed_updated(&self, old: ObjectPath<'_>, new: ObjectPath<'_>) -> Result<()>;
}

#[proxy(
    default_service = "org.freedesktop.GeoClue2",
    interface = "org.freedesktop.GeoClue2.Location"
)]
trait Location {
    #[zbus(property)]
    fn latitude(&self) -> zbus::Result<f64>;
    #[zbus(property)]
    fn longitude(&self) -> zbus::Result<f64>;
}

#[tokio::main]
async fn main() -> zbus::Result<()> {
    let conn = Connection::system().await?;
    let manager = ManagerProxy::new(&conn).await?;
    let mut client = manager.get_client().await?;

    client.set_desktop_id("org.freedesktop.zbus").await?;

    let props = zbus::fdo::PropertiesProxy::builder(&conn)
        .destination("org.freedesktop.GeoClue2")?
        .path(client.inner().path())?
        .build()
        .await?;
    let mut props_changed = props.receive_properties_changed().await?;
    let mut location_updated = client.receive_locationed_updated().await?;

    client.start().await?;

    futures_util::try_join!(
        async {
            while let Some(signal) = props_changed.next().await {
                let args = signal.args()?;
                for (name, value) in args.changed_properties().iter() {
                    println!(
                        "{}.{} changed to `{:?}`",
                        args.interface_name(),
                        name,
                        value
                    );
                }
            }

            Ok::<(), zbus::Error>(())
        },
        async {
            while let Some(signal) = location_updated.next().await {
                let args = signal.args()?;

                let location = LocationProxy::builder(&conn)
                    .path(args.new())?
                    .build()
                    .await?;
                println!(
                    "LAT: {}\nLONG: {}",
                    location.latitude().await?,
                    location.longitude().await?
                );
            }

            Ok(())
        }
    )?;

    Ok(())
}
