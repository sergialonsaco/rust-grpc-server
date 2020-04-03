use tonic::{metadata::MetadataValue, Request, Status};

pub fn interceptor(req: Request<()>) -> Result<Request<()>, Status> {
    let token = MetadataValue::from_str("Bearer myjwttoken").unwrap();
    match req.metadata().get("authorization") {
        Some(t) if t == token => {
            println!("Intercepting the request");
            Ok(req)
        }
        _ => Err(Status::unauthenticated("The token is not valid")),
    }
}
