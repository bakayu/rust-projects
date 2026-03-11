use hello::HelloRequest;
use hello::greeter_client::GreeterClient;
use tonic::Request;

pub mod hello {
    tonic::include_proto!("hello.v1");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let reqest = Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(reqest).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
