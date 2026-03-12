// MousePreferenceClient - generated client stub, handles serialization and the gRPC connection
use mouse::PreferenceReport;
use mouse::mouse_preference_client::MousePreferenceClient;

// Pulls in the Rust types that tonic-build generated from mouse.proto at build time
pub mod mouse {
    tonic::include_proto!("mouse");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Establishes the underlying HTTP/2 connection to the server
    let mut client = MousePreferenceClient::connect("http://[::1]:50051").await?;

    let reports = vec![
        PreferenceReport {
            name: "Reze".into(),
            is_city_mouse: false,
        },
        PreferenceReport {
            name: "Denji".into(),
            is_city_mouse: true,
        },
        PreferenceReport {
            name: "Aki".into(),
            is_city_mouse: false,
        },
    ];

    // tokio_stream::iter - wraps a Vec into an async stream,
    // which is what Tonic expects for client streaming RPCs
    let stream = tokio_stream::iter(reports);

    // Sends the entire stream in a single gRPC connection, awaits the server's response
    let response = client.report_preference(stream).await?;

    println!("Server said: {}", response.into_inner().message);

    Ok(())
}
