#![allow(unused_imports)]

extern crate dotenv;
extern crate grpc;
extern crate protobuf;
extern crate redis;
extern crate rustracing;
extern crate rustracing_jaeger;
extern crate tls_api;

use std::env;
use std::thread;

use dotenv::dotenv;
use tls_api::TlsAcceptorBuilder;
use grpc::*;

use geofancy_server::GeofancyImpl;

use geofancy_grpc::*;

fn main() {
    let mut server = grpc::ServerBuilder::new_plain();

    dotenv().ok();

    let grpc_server_port: u16 = u64::from_str_radix(env::var("GRPC_SERVER_PORT").unwrap().as_str(), 10).unwrap() as u16;

    server.http.set_port(grpc_server_port);

    server.add_service(GeofancyServiceServer::new_service_def(GeofancyImpl));
    server.http.set_cpu_pool_threads(2);

    let _server = server.build().expect("server");

    println!("geofancy server started on port {}", grpc_server_port);

    loop {
        thread::park();
    }
}

mod geofancy_server;
mod geofancy;
mod geofancy_grpc;
mod tile38_client;