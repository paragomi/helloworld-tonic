use tonic::{transport::Server, Request, Response, Status};

use hello_world::configuration_server::{Configuration, ConfigurationServer};
use hello_world::{MpcParameters};

pub mod hello_world {
    tonic::include_proto!("mpcconfig");
}

#[derive(Debug, Default)]
pub struct MpcConfiguration {}

#[tonic::async_trait]
impl Configuration for MpcConfiguration {
    async fn set_parameters(
        &self,
        request: Request<MpcParameters>,
    ) -> Result<Response<MpcParameters>, Status> {
        println!("Got a request: {:?}", request);

        Ok(Response::new(request.into_inner()))
    }

    async fn get_parameters(
        &self,
        _request: Request<()>
    ) -> Result<Response<MpcParameters>, Status> {
        let parameters = hello_world::MpcParameters{
            number_of_party : 1,
            signing_count : 2,
            resigning_count : 3,
        };

        Ok(Response::new(parameters))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let configuration = MpcConfiguration::default();

    Server::builder()
        .add_service(ConfigurationServer::new(configuration))
        .serve(addr)
        .await?;

    Ok(())
}