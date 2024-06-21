fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/geofancy.proto");
    std::env::set_var("OUT_DIR", "src");
    tonic_build::configure()
        .build_server(true)
        .compile(&["proto/geofancy.proto"], &["proto/"])?;
    Ok(())
}
