use crate::proto::{server01_server::Server01, HiRequest, HiResponse};
use tonic::{Request, Response, Status};

#[allow(clippy::module_name_repetitions)]
pub struct GrpcServer {}

#[tonic::async_trait]
impl Server01 for GrpcServer {
    async fn say_hi(&self, request: Request<HiRequest>) -> Result<Response<HiResponse>, Status> {
        println!("{:?}", request);
        let response = HiResponse {
            message: format!("Hello {}", request.into_inner().hello),
        };
        Ok(Response::new(response))
    }
}
