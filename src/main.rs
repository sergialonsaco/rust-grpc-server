mod interceptor;
mod proto;
mod service;

use interceptor::interceptor;
use proto::server01_server::Server01Server;
use service::GrpcServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address: &str = "0.0.0.0:50051";
    let addr = address.parse()?;
    let rpts01_service = GrpcServer {};

    Server::builder()
        .add_service(Server01Server::with_interceptor(
            rpts01_service,
            interceptor,
        ))
        .serve(addr)
        .await?;

    Ok(())
}
