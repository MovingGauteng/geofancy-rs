extern crate tower_grpc_build;

fn main() {
    std::env::set_var("OUT_DIR", "src/");

    tower_grpc_build::Config::new()
        .enable_server(true)
        .build(&["proto/geofancy.proto"], &["proto"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
}