// tonic::transport::Server - the gRPC server builder
// MousePreferenceServer - generated wrapper that registers our service with the server
use mouse::mouse_preference_server::{MousePreference, MousePreferenceServer};
use mouse::{PreferenceReport, PreferenceSummary};
use tonic::{Request, Response, Status, Streaming, transport::Server};

// Pulls in the Rust types that tonic-build generated from mouse.proto at build time
pub mod mouse {
    tonic::include_proto!("mouse");
}

#[derive(Default)]
pub struct MousePreferenceService;

#[tonic::async_trait] // required - tonic needs async trait support via this macro
impl MousePreference for MousePreferenceService {
    async fn report_preference(
        &self,
        // Streaming<T> - tonic's type for an incoming client stream of messages
        request: Request<Streaming<PreferenceReport>>,
    ) -> Result<Response<PreferenceSummary>, Status> {
        let mut stream = request.into_inner();
        let mut city_count = 0;
        let mut country_count = 0;

        // .message().await? - pulls the next message off the gRPC stream,
        // returns None when the client closes the stream
        while let Some(report) = stream.message().await? {
            match report.is_city_mouse {
                true => {
                    city_count += 1;
                    println!("{} is a city mouse.", report.name);
                }
                false => {
                    country_count += 1;
                    println!("{} is a country mouse.", report.name);
                }
            }
        }

        // Respond once with the final tally
        Ok(Response::new(PreferenceSummary {
            received: true,
            message: format!("Results: {} city, {} country.", city_count, country_count),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = MousePreferenceService::default();

    println!("Server listening on {}", addr);

    // .add_service() - registers our generated service wrapper with the gRPC server
    Server::builder()
        .add_service(MousePreferenceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
