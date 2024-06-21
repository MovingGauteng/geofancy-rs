use std::env;

use tonic::transport::Server;

use dotenv::dotenv;

use geofancy_server::GeofancyImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv();

    let grpc_uri = env::var("GRPC_SERVER_URI")?;

    let handler = GeofancyImpl {};

    let addr = &grpc_uri.parse().unwrap();
    let new_service = geofancy::geofancy_service_server::GeofancyServiceServer::new(handler);

    println!("listening on {:?}", addr);

    Server::builder()
        .add_service(new_service)
        .serve(*addr)
        .await?;

    Ok(())
}

mod geofancy;
mod geofancy_server;
mod tile38_client;
