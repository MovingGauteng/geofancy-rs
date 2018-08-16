// #![allow(unused_imports)]

#[macro_use]
extern crate log;
extern crate dotenv;
extern crate prost;
#[macro_use]
extern crate prost_derive;
extern crate redis;
extern crate rustracing;
extern crate rustracing_jaeger;
extern crate tokio;
extern crate futures;
extern crate tokio_core;
extern crate tower_h2;
extern crate tower_grpc;

use std::env;

use futures::{Stream, Future};

use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use tower_h2::Server;

use dotenv::dotenv;

use geofancy_server::GeofancyImpl;

fn main() {

    dotenv().ok();

    let grpc_server_port = env::var("GRPC_SERVER_PORT").unwrap();

    let mut core = Core::new().unwrap();
    let reactor = core.handle();

    let handler = GeofancyImpl {};

    let new_service = geofancy::server::GeofancyServiceServer::new(handler);

    let h2 = Server::new(new_service, Default::default(), reactor.clone());

    let a = format!("127.0.0.1:{}", &grpc_server_port);

    let addr = a.parse().unwrap();
    let bind = TcpListener::bind(&addr, &reactor).expect("bind");

    println!("listening on {:?}", addr);

    let serve = bind.incoming()
        .fold((h2, reactor), |(h2, reactor), (sock, _)| {
            if let Err(e) = sock.set_nodelay(true) {
                return Err(e);
            }

            let serve = h2.serve(sock);
            reactor.spawn(serve.map_err(|e| error!("h2 error: {:?}", e)));

            Ok((h2, reactor))
        });

    core.run(serve).unwrap();
}

mod geofancy_server;
mod geofancy;
mod tile38_client;